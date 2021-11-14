# rust-tokio-template

This template is provides a subsystem-based tokio project.

## Features

The following features get set up by this template:

- Command line argument parser (clap)
- Logging (env_logger)
- Return code error propagation (anyhow)
- Tokio runtime
- Ctrl-Z & SIGTERM handling
- Graceful subsystem shutdown

Especially graceful shutdown is not a trivial task in Async Rust.
The code needed to perform an async shutdown is big enough that it might get moved to its own crate at some point.

## Usage

Install `cargo-generate`:
```
cargo install cargo-generate
```

Generate into a subfolder:
```
cargo generate --git https://github.com/Finomnis/rust-tokio-template.git
```

Generate into the current folder:
```
cargo generate --init --git https://github.com/Finomnis/rust-tokio-template.git
```
