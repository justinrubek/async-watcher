use async_watcher::{
    notify::{EventKind, RecursiveMode},
    AsyncDebouncer,
};
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
    while let Some(Ok(events)) = file_events.recv().await {
        events.iter().for_each(|e| display_event(&e.event));
    }

    Ok(())
}

fn display_event(event: &async_watcher::notify::Event) {
    let action = match event.kind {
        EventKind::Access(_) => "accessed",
        EventKind::Any => "any",
        EventKind::Create(_) => "created",
        EventKind::Modify(_) => "modified",
        EventKind::Other => "other",
        EventKind::Remove(_) => "removed",
    };
    println!("{}: {:?}", action, event.paths);
}
