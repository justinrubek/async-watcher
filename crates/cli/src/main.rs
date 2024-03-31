use crate::error::Result;
use async_walkdir::WalkDir;
use async_watcher::{notify::RecursiveMode, AsyncDebouncer};
use clap::Parser;
use globset::{Glob, GlobSetBuilder};
use std::time::Duration;
use tokio_stream::StreamExt;
use tracing::{debug, info};

pub mod commands;
pub mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    info!(?args);

    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(&args.glob)?);
    let globset = builder.build()?;

    let paths = WalkDir::new(".")
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();
                if globset.is_match(&path) {
                    debug!(?path, "matched");
                    Some(Ok(path))
                } else {
                    debug!(?path, "did not match");
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect::<std::result::Result<Vec<_>, _>>()
        .await?;
    info!(?paths, "matched against");

    let (mut debouncer, mut file_events) =
        AsyncDebouncer::new_with_channel(Duration::from_secs(1), Some(Duration::from_secs(1)))
            .await?;

    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    let mut build_process = match args.skip_launch_on_startup {
        true => None,
        false => Some(
            tokio::process::Command::new(&args.command)
                .args(&args.args)
                .spawn()
                .expect("failed to spawn child process"),
        ),
    };

    while let Some(event) = file_events.recv().await {
        match event {
            Ok(events) => {
                info!(?events, "detected changes, restarting command");

                if let Some(ref mut child) = build_process {
                    child.kill().await.expect("failed to kill child process");
                }

                let process = tokio::process::Command::new(&args.command)
                    .args(&args.args)
                    .spawn()
                    .expect("failed to spawn child process");

                build_process = Some(process);
            }
            Err(errors) => {
                for error in errors {
                    tracing::error!(?error, "error watching file");
                }
            }
        }
    }

    Ok(())
}
