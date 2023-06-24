use std::{path::Path, time::Duration};

use async_watcher::{notify::RecursiveMode, AsyncDebouncer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = vec!["Cargo.toml", "Cargo.lock", "crates", "examples"];

    async_debounce_watch(paths).await?;

    Ok(())
}

pub async fn async_debounce_watch<P: AsRef<Path>>(
    paths: Vec<P>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut debouncer =
        AsyncDebouncer::new(Duration::from_secs(1), Some(Duration::from_secs(1)), tx).await?;

    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    // keep track of the child process so we can kill it later
    let mut build_process: Option<tokio::process::Child> = None;

    while let Some(event) = rx.recv().await {
        match event {
            Ok(_events) => {
                // for this example we are triggering on any event, so we are not checking the info
                println!("Detected changes in files, rebuilding project");

                if let Some(ref mut child) = build_process {
                    println!("killing child process");
                    child.kill().await.expect("failed to kill child process");
                }

                println!("building cargo project");
                let process = tokio::process::Command::new("cargo")
                    .arg("build")
                    .spawn()
                    .expect("failed to spawn child process");

                build_process = Some(process);
            }
            Err(errors) => {
                for error in errors {
                    println!("error: {error:?}");
                }
            }
        }
    }

    Ok(())
}
