use clap::Parser;
use env_logger::Env;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Options {
    /// Increase verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
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
