mod cli;
mod logger;

use clap::Parser;
pub(crate) use cli::Cli;
pub(crate) use logger::init_logger;

/// Initialize the application services.
pub(super) fn setup() -> anyhow::Result<Cli> {
    let cli = Cli::parse();
    init_logger(&cli)?;
    Ok(cli)
}
