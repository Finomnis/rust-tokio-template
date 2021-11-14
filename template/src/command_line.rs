use clap::{crate_version, Parser};
use env_logger::Env;

// This doc string acts as a help message when the user runs '--help'
// as do all doc strings on fields
#[derive(Parser)]
#[clap(version = crate_version!())]
pub struct Options {
    /// Increase verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

pub fn parse() -> Options {
    let opts = Options::parse();

    let debug_level = match opts.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(debug_level)).init();

    opts
}
