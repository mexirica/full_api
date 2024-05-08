use log4rs::append::{console, file};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use tracing::log;

pub fn configure_log() -> log4rs::Config  {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
            .build("log/requests.log")
            .unwrap();

             Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("requests", Box::new(requests)))
            .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
            .logger(Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", LevelFilter::Info))
            .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
            .unwrap()
}

