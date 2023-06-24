# async-watcher

This is a small library that uses [notify][https://github.com/notify-rs/notify] to implement a file watcher that is debounced.
Notify takes care of all of the work, but when I was implementing it in my application I noticed that there wasn't an async implementation.
This implementation was extracted from one that I used on another project that needed this functionality but used tokio.

See the [examples](./examples) for usage. The library defines an event handler async_trait and implements it for tokio::sync::mpsc.
