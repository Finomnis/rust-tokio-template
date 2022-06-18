# rust-tokio-template

This template is provides a subsystem-based tokio project.

## Features

The following features get set up by this template:

- Command line argument parser (clap)
- Logging (env_logger)
- Return code error propagation (miette)
- Tokio runtime
- Ctrl-C & SIGTERM handling
- Graceful subsystem shutdown

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
