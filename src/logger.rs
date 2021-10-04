use std::path::Path;

pub use log::{debug, error, info, warn, LevelFilter};
pub use log4rs;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::{json::JsonEncoder, pattern::PatternEncoder, Encode},
    Config,
};

use crate::cli::Opt;
use crate::error::*;

const ENCODER_PATTERN: &str = "\x1b[1m[{l}]\x1b[0m {m}\n";

/// Construct a new encoder.
fn new_encoder(json_log: Option<bool>) -> Box<dyn Encode> {
    match json_log {
        Some(v) => match v {
            true => Box::new(JsonEncoder::new()),
            false => Box::new(PatternEncoder::new(ENCODER_PATTERN)),
        },
        None => Box::new(PatternEncoder::new(ENCODER_PATTERN)),
    }
}

/// The base context of `get_*_config`.
struct GetConfigBase {
    encoder: Box<dyn Encode>,
    log_level: LevelFilter,
}

/// Get the configuration for logging to stdout.
fn get_stdout_config(conf_base: GetConfigBase) -> Result<Config> {
    let GetConfigBase { encoder, log_level } = conf_base;

    let stdout = ConsoleAppender::builder().encoder(encoder).build();
    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level))?;
    Ok(config)
}

/// Get the configuration for logging to a file.
fn get_log_path_config(conf_base: GetConfigBase, log_path: &Path) -> Result<Config> {
    let GetConfigBase { encoder, log_level } = conf_base;

    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(encoder)
        .build(log_path)
        .unwrap();

    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(log_level))?;
    Ok(config)
}

/// Initiate the logger.
pub fn init_logger(setting: &Opt) -> Result<()> {
    let cfg_ctx = GetConfigBase {
        encoder: new_encoder(setting.env.json_log),
        log_level: setting.env.log_level,
    };

    let log_config = match &setting.env.log_file {
        None => get_stdout_config(cfg_ctx),
        Some(log_path) => get_log_path_config(cfg_ctx, log_path),
    }?;

    log4rs::init_config(log_config).map_err(|e| Error::LogSetupFailed(format!("{}", e)))?;
    Ok(())
}
