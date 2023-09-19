use async_watcher::{
    notify::{self, RecommendedWatcher, RecursiveMode},
    AsyncDebouncer, DebouncedEvent,
};
use std::{path::Path, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = vec!["Cargo.toml", "Cargo.lock", "crates", "examples"];

    let (mut file_events, _debouncer) = async_debounce_watch(paths).await?;

    while let Some(event) = file_events.recv().await {
        println!("event: {:?}", event);
    }

    Ok(())
}

pub async fn async_debounce_watch<P: AsRef<Path>>(
    paths: Vec<P>,
) -> Result<
    (
        tokio::sync::mpsc::Receiver<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
        AsyncDebouncer<RecommendedWatcher>,
    ),
    Box<dyn std::error::Error>,
> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    let mut debouncer =
        AsyncDebouncer::new(Duration::from_secs(1), Some(Duration::from_secs(1)), tx).await?;

    // add the paths to the watcher
    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    Ok((rx, debouncer))
}
