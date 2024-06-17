use anyhow::Context;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
    Config,
};

use super::Cli;

/// Initializes the logger based on the CLI configuration.
pub(crate) fn init_logger(cli: &Cli) -> anyhow::Result<()> {
    let stdout_builder = ConsoleAppender::builder();

    let stdout = match &cli.log_encoder {
        super::cli::LogEncoder::Json => stdout_builder.encoder(Box::new(JsonEncoder::new())),
        super::cli::LogEncoder::Text => stdout_builder.encoder(Box::new(
            log4rs::encode::pattern::PatternEncoder::new("{d} {l} {t} - {m}{n}"),
        )),
    }
    .build();

    let log_level = cli.log_level.clone().into();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level))
        .context("Configuring logger")?;

    log4rs::init_config(config).context("Initializing logger")?;
    Ok(())
}
