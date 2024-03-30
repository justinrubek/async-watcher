#![feature(async_closure)]
use crate::error::Result;
use async_walkdir::WalkDir;
use async_watcher::{notify::RecursiveMode, AsyncDebouncer};
use clap::Parser;
use globset::{Glob, GlobSetBuilder};
use std::{process::ExitStatus, time::Duration};
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

    let mut task = tokio::spawn(run_command(args.command.clone(), args.args.clone()));

    loop {
        tokio::select! {
            events = file_events.recv() => {
                match events {
                    Some(events) => {
                        info!(?events, "file changed, restarting command");

                        task.abort();
                        task = tokio::spawn(run_command(args.command.clone(), args.args.clone()));
                    }
                    None => {
                        tracing::error!("file watcher channel closed, exiting");
                        break;
                    }
                }
            }
            result = &mut task => {
                info!(?result, "command exited");
                result??;
            }
        }
    }

    Ok(())
}

async fn run_command(command: String, args: Vec<String>) -> Result<ExitStatus> {
    let mut task = tokio::process::Command::new(command).args(args).spawn()?;
    let result = task.wait().await?;
    Ok(result)
}
