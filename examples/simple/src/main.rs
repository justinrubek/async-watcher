use async_watcher::{notify::RecursiveMode, AsyncDebouncer};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize the debouncer
    let (mut debouncer, mut file_events) =
        AsyncDebouncer::new_with_channel(Duration::from_secs(1), Some(Duration::from_secs(1)))
            .await?;

    // register the paths to be watched
    let paths = ["Cargo.toml", "Cargo.lock", "crates", "examples"];
    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    // wait for events
    while let Some(event) = file_events.recv().await {
        println!("event: {:?}", event);
    }

    Ok(())
}
