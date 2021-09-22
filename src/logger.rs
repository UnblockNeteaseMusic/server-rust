use crate::cli::Opt;
pub use log::{debug, error, info, warn, LevelFilter};
pub use log4rs;

use log4rs::{
	append::{console::ConsoleAppender, file::FileAppender},
	config::{Appender, Root},
	encode::{json::JsonEncoder, pattern::PatternEncoder, Encode},
};

/// initilizate logger
pub fn init_logger(setting: &Opt) {
	let encoder: Box<dyn Encode> = match setting.env.json_log {
		Some(v) => match v {
			true => Box::new(JsonEncoder::new()),
			false => Box::new(PatternEncoder::new("{l} - {m}\n")),
		},
		None => Box::new(PatternEncoder::new("{l} - {m}\n")),
	};

	let log_config = match &setting.env.log_file {
		None => {
			// use stdout as outputs
			let stdout: ConsoleAppender = ConsoleAppender::builder().encoder(encoder).build();
			log4rs::config::Config::builder()
				.appender(Appender::builder().build("stdout", Box::new(stdout)))
				.build(
					Root::builder()
						.appender("stdout")
						.build(setting.env.log_level),
				)
		}
		Some(log_path) => {
			// Logging to log file.
			let logfile = FileAppender::builder()
				// Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
				.encoder(encoder)
				.build(log_path.to_str().unwrap()) // TODO: check if path is exists and to_str is Some
				.unwrap();
			log4rs::config::Config::builder()
				.appender(Appender::builder().build("logfile", Box::new(logfile)))
				.build(
					Root::builder()
						.appender("logfile")
						.build(setting.env.log_level),
				)
		}
	};
	log4rs::init_config(log_config.unwrap()).unwrap();
}
