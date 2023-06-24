use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

pub use notify;
use notify::{Error as NotifyError, Event, RecommendedWatcher, Watcher};

pub mod error;

/// Deduplicate event data
struct EventData {
    /// Insertion Time
    insert: Instant,
    /// Last Update
    update: Instant,
}

impl EventData {
    fn new_any() -> Self {
        let time = Instant::now();
        Self {
            insert: time,
            update: time,
        }
    }
}

/// The types of events that can be debounced.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DebouncedEventKind {
    /// No precise events
    Any,
    /// Event where debounce timed out (for example continuous writes)
    AnyContinuous,
}

/// A debounced event.
///
/// Does not emit any specific event type on purpose, only distinguishes between an any event and a continuous any event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebouncedEvent {
    /// Event path
    pub path: PathBuf,
    /// Event kind
    pub kind: DebouncedEventKind,
}

impl DebouncedEvent {
    fn new(path: PathBuf, kind: DebouncedEventKind) -> Self {
        Self { path, kind }
    }
}

type DebounceData = Arc<Mutex<DebounceDataInner>>;

#[derive(Default)]
struct DebounceDataInner {
    d: HashMap<PathBuf, EventData>,
    timeout: Duration,
    e: Vec<NotifyError>,
}

impl DebounceDataInner {
    /// Retrieve a vec of debounced events, removing them if they are not continuous
    pub fn debounced_events(&mut self) -> Vec<DebouncedEvent> {
        let mut events_expired = Vec::with_capacity(self.d.len());
        let mut data_back = HashMap::with_capacity(self.d.len());

        // TODO: drain_filter https://github.com/rust-lang/rust/issues/59618
        for (k, v) in self.d.drain() {
            if v.update.elapsed() >= self.timeout {
                events_expired.push(DebouncedEvent::new(k, DebouncedEventKind::Any));
            } else if v.insert.elapsed() >= self.timeout {
                data_back.insert(k.clone(), v);
                events_expired.push(DebouncedEvent::new(k, DebouncedEventKind::AnyContinuous));
            } else {
                data_back.insert(k, v);
            }
        }

        self.d = data_back;
        events_expired
    }

    /// Takes all currently stored errors
    pub fn errors(&mut self) -> Vec<NotifyError> {
        std::mem::take(&mut self.e)
    }

    /// Add an error entry to re-send later on
    pub fn add_error(&mut self, e: NotifyError) {
        self.e.push(e);
    }

    /// Add new event to debouncer cache
    pub fn add_event(&mut self, e: Event) {
        for path in e.paths.into_iter() {
            if let Some(v) = self.d.get_mut(&path) {
                v.update = Instant::now();
            } else {
                self.d.insert(path, EventData::new_any());
            }
        }
    }
}

#[async_trait::async_trait]
pub trait AsyncDebounceEventHandler {
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<NotifyError>>);
}

#[async_trait::async_trait]
impl<F> AsyncDebounceEventHandler for F
where
    F: FnMut(Result<Vec<DebouncedEvent>, Vec<NotifyError>>) + Send + 'static,
{
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<NotifyError>>) {
        self(event)
    }
}

#[async_trait::async_trait]
impl AsyncDebounceEventHandler
    for tokio::sync::mpsc::Sender<Result<Vec<DebouncedEvent>, Vec<NotifyError>>>
{
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<NotifyError>>) {
        let _ = self.send(event).await;
    }
}

pub struct AsyncDebouncer<T: Watcher> {
    stop: Arc<AtomicBool>,
    watcher: T,
    debouncer_task: Option<tokio::task::JoinHandle<()>>,
}

impl<T: Watcher> AsyncDebouncer<T> {
    pub async fn stop(mut self) {
        self.set_stop();
        if let Some(t) = self.debouncer_task.take() {
            let _ = t.await;
        }
    }

    fn set_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn watcher(&mut self) -> &mut dyn Watcher {
        &mut self.watcher
    }
}

impl<T: Watcher> Drop for AsyncDebouncer<T> {
    fn drop(&mut self) {
        // don't block on drop
        self.set_stop();
    }
}

impl<T: Watcher> AsyncDebouncer<T> {
    /// Creates a new debounced watcher with custom configuration.
    /// The timeout specifies the amount of time that must elapse before an event is emitted, or a
    /// continuous event is sent (if there still are events incoming for the specific path).
    /// If tick_rate is set to None, then a tick rate will be selected that is less than the provided timeout.
    pub async fn new_with_opts<F: AsyncDebounceEventHandler + Send + 'static>(
        timeout: Duration,
        tick_rate: Option<Duration>,
        mut event_handler: F,
        config: notify::Config,
    ) -> Result<Self, Error> {
        let data = DebounceData::default();

        let stop = Arc::new(AtomicBool::new(false));

        let tick_div = 4;
        let tick = match tick_rate {
            Some(v) => {
                if v > timeout {
                    return Err(Error::InvalidTickRate(v, timeout));
                }
                v
            }
            None => timeout
                .checked_div(tick_div)
                .ok_or_else(|| Error::FailedToCalculateTick(timeout, tick_div))?,
        };

        {
            let mut data_w = data.lock().unwrap();
            data_w.timeout = timeout;
        }

        let data_c = data.clone();
        let stop_c = stop.clone();
        let debouncer_task = tokio::spawn(async move {
            loop {
                if stop_c.load(Ordering::Acquire) {
                    break;
                }
                tokio::time::sleep(tick).await;
                let send_data;
                let errors: Vec<NotifyError>;
                {
                    let mut lock = data_c.lock().expect("can't lock debouncer data");
                    send_data = lock.debounced_events();
                    errors = lock.errors();
                }
                if !send_data.is_empty() {
                    event_handler.handle_event(Ok(send_data)).await;
                }
                if !errors.is_empty() {
                    event_handler.handle_event(Err(errors)).await;
                }
            }
        });

        let watcher = T::new(
            move |e: Result<Event, NotifyError>| {
                let mut lock = data.lock().expect("can't lock debouncer data");

                match e {
                    Ok(e) => lock.add_event(e),
                    // errors are stored and sent later on
                    Err(e) => lock.add_error(e),
                }
            },
            config,
        )?;

        let guard = AsyncDebouncer {
            watcher,
            debouncer_task: Some(debouncer_task),
            stop,
        };

        Ok(guard)
    }
}

impl AsyncDebouncer<RecommendedWatcher> {
    /// Creates a new debounced watcher with the recommended watcher implementation.
    /// The timeout specifies the amount of time that must elapse before an event is emitted, or a
    /// continuous event is sent (if there still are events incoming for the specific path).
    /// If tick_rate is set to None, then a tick rate will be selected that is less than the provided timeout.
    pub async fn new<F: AsyncDebounceEventHandler + Send + 'static>(
        timeout: Duration,
        tick_rate: Option<Duration>,
        event_handler: F,
    ) -> Result<Self, Error> {
        AsyncDebouncer::new_with_opts(timeout, tick_rate, event_handler, notify::Config::default())
            .await
    }
}
