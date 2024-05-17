# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [0.3.0](https://github.com/justinrubek/async-watcher/releases/tag/0.3.0)

This is a small change that adds the `event` field to `DebouncedEvent`.
This field is the event from `notify`, which allows for more advanced use cases.

### build system
- **(nix)** update fenix input - ([438f0ab](https://github.com/justinrubek/async-watcher/commit/438f0aba8fa3690afbe52146e5a3c49c5b871adf)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** cargo update - ([c8c3080](https://github.com/justinrubek/async-watcher/commit/c8c3080e4230d20e2253a30b61dc606c7d79df33)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update bomper - ([9638d84](https://github.com/justinrubek/async-watcher/commit/9638d84c67446412a9c90437468b8e51a5a62156)) - [@justinrubek](https://github.com/justinrubek)

### chores
- remove async_closure rust feature - ([6b68a00](https://github.com/justinrubek/async-watcher/commit/6b68a00b2644048a51b08c3377ad0516a7612160)) - [@justinrubek](https://github.com/justinrubek)

### continuous integration
- **(github/actions)** use nix to acquire rust toolchain - ([a7be0f6](https://github.com/justinrubek/async-watcher/commit/a7be0f6b84a4407e9a4cfc11f18f3c89959c665c)) - [@justinrubek](https://github.com/justinrubek)
- **(github/actions)** replace cocogitto with bomper - ([9fd3224](https://github.com/justinrubek/async-watcher/commit/9fd32242211bed17f3113b76c137e9585188b3db)) - [@justinrubek](https://github.com/justinrubek)

### features
- add `notify::Event` event to `DebouncedEvent` - ([0b4431f](https://github.com/justinrubek/async-watcher/commit/0b4431ff7359f5ddb8546262bba0cb97f0810578)) - [@justinrubek](https://github.com/justinrubek)
- **(cli)** add ability to skip command invocation on startup - ([9a518e4](https://github.com/justinrubek/async-watcher/commit/9a518e45a97c2b0f94c54b8778a69fd1d05ac31c)) - [@justinrubek](https://github.com/justinrubek)

### refactors
- **(examples/simple)** access event data and pretty-print - ([b659eff](https://github.com/justinrubek/async-watcher/commit/b659eff2642e194d0ac4b1a9a0a0a13ccbca1dfe)) - [@justinrubek](https://github.com/justinrubek)
- **(cli)** vastly simplify child process logic - ([3bcd9c3](https://github.com/justinrubek/async-watcher/commit/3bcd9c3785a76638eced99fd69ce2cf9f24d3cdd)) - [@justinrubek](https://github.com/justinrubek)

- - -
## [0.2.1](https://github.com/justinrubek/async-watcher/compare/5ad94fea49d19cf12c5fc19aeedfe4ba5d55ab2f..0.2.1) - 2024-03-30
#### Bug Fixes
- **(cli)** support commands that exit - ([00056f2](https://github.com/justinrubek/async-watcher/commit/00056f2cc88535c96ecc306e57322ab25268223d)) - [@justinrubek](https://github.com/justinrubek)
#### Build system
- **(cargo)** cargo update - ([1e468a0](https://github.com/justinrubek/async-watcher/commit/1e468a058194572776c5e7f211e49287dd76a9d2)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** add `awatch` package - ([df17dff](https://github.com/justinrubek/async-watcher/commit/df17dff798e6a47db9938420e7bf207fd65b4318)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** nix flake update - ([60f5819](https://github.com/justinrubek/async-watcher/commit/60f58198a6f69d69623a4a48c6114dc3634ad308)) - [@justinrubek](https://github.com/justinrubek)
#### Documentation
- **(readme)** add cli information - ([251d1ef](https://github.com/justinrubek/async-watcher/commit/251d1ef27880aa8ed8f79fb6caaf8a96b5f8d7f9)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- add `awatch` cli - ([a6469d2](https://github.com/justinrubek/async-watcher/commit/a6469d2da459bf22f57f33d955bd6bf0864b3b1e)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- **(examples)** remove useless vec usage - ([5ad94fe](https://github.com/justinrubek/async-watcher/commit/5ad94fea49d19cf12c5fc19aeedfe4ba5d55ab2f)) - [@justinrubek](https://github.com/justinrubek)

- - -

## [0.2.0](https://github.com/justinrubek/async-watcher/compare/0.1.1..0.2.0) - 2023-09-19
#### Documentation
- **(example)** add channel example - ([17fba9f](https://github.com/justinrubek/async-watcher/commit/17fba9fc5f8dd9481b82336306c3292c37bf50ee)) - [@justinrubek](https://github.com/justinrubek)
- **(examples)** greatly simplify the `simple` example - ([8551587](https://github.com/justinrubek/async-watcher/commit/85515876715952addfc2ca789671da5eb6ef5b67)) - [@justinrubek](https://github.com/justinrubek)
- **(examples/rebuild)** utilize AsyncDebouncer::new_with_channel - ([f6bf758](https://github.com/justinrubek/async-watcher/commit/f6bf758cca54288a3f5f8b6b1896ace53de54b83)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- add debounce functions that use a tokio::sync::mpsc::channel - ([3ea3fb7](https://github.com/justinrubek/async-watcher/commit/3ea3fb790ecc0af580f7d1e8580c26e11b5d24ef)) - [@justinrubek](https://github.com/justinrubek)

- - -

## [0.1.1](https://github.com/justinrubek/async-watcher/compare/0.1.0..0.1.1) - 2023-06-24
#### Build system
- **(cargo)** add readme - ([6fe2322](https://github.com/justinrubek/async-watcher/commit/6fe23223df2aa2ff13ac119eaa8fa37ff41864b7)) - [@justinrubek](https://github.com/justinrubek)
#### Documentation
- **(readme)** add badges - ([4ccdc4e](https://github.com/justinrubek/async-watcher/commit/4ccdc4ed69870a9d1aa64031206441da12ebd1bc)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- add bomper and cog configuration - ([f61e783](https://github.com/justinrubek/async-watcher/commit/f61e783c13766422e8bdc28e69f91c854919b2e4)) - [@justinrubek](https://github.com/justinrubek)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).