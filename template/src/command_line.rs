use clap::Parser;

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
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(debug_level).init();

    opts
}
