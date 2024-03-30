# async-watcher

[<img alt="github" src="https://img.shields.io/badge/github-justinrubek/async--watcher-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/justinrubek/async-watcher)
[<img alt="crates.io" src="https://img.shields.io/crates/v/async-watcher.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/async-watcher)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-async--watcher-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/async-watcher)

This is a small library that uses [notify](https://github.com/notify-rs/notify) to implement a file watcher that is debounced.
The debouncing helps ensure you don't get too many events on a single file leading to extra work.
The original use case of this was to watch a directory and rebuild when there are changes.
Without debouncing, a single file could trigger multiple rebuilds.
Instead, `async-watcher` can be used to trigger events after a specific time threshold.

Notify takes care of all of the work, but the debouncing traits weren't async.
`async-watcher` leverages [async-trait](https://github.com/dtolnay/async-trait) to provide an async interface for creating watchers.

See the [examples](./examples) for usage.

## cli

Included is a command line application that can be used to execute a command when notified of changes.
The files to watch are specified with a glob pattern as the first argument.
Subsequent arguments are the command to run and its arguments.
The program is exposed as a nix flake package called `awatch`.

example usage:
```sh
awatch './{crates,Cargo.toml,Cargo.lock}' echo "crates modified"
```
