# async-watcher

This is a small library that uses [notify](https://github.com/notify-rs/notify) to implement a file watcher that is debounced.
The debouncing helps ensure you don't get too many events on a single file leading to extra work.
The original use case of this was to watch a directory and rebuild when there are changes.
Without debouncing, a single file could trigger multiple rebuilds.
Instead, `async-watcher` can be used to trigger events after a specific time threshold.

Notify takes care of all of the work, but the debouncing traits weren't async.
`async-watcher` leverages [async-trait](https://github.com/dtolnay/async-trait) to provide an async interface for creating watchers.

See the [examples](./examples) for usage.
