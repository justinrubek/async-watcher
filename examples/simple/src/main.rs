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

    while let Some(event) = rx.recv().await {
        match event {
            Ok(events) => {
                events.iter().for_each(|e| println!("event: {:?}", e));
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
