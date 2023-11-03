# rust-tokio-template

This template generates a subsystem-based tokio project ready to be compiled.

## Features

The following features get set up by this template:

- Command line argument parser (clap)
- Logging (tracing)
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
