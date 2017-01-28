use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use time;
use tantivy;

use ::server;

pub fn start() {
    init_logging();
    info!("Starting up (Version: {}, tantivy version: {})",
          env!("CARGO_PKG_VERSION"),
          tantivy::version());
    start_server();

}

fn init_logging() {
    let format = |record: &LogRecord| {
        let t = time::now_utc();
        let location = record.location();
        format!(
            "{}.{:03} {:4.4} {} [{:10.10}|{:.15}:{}]",
            time::strftime("%Y-%m-%d %H:%M:%S", &t).unwrap(),
            t.tm_nsec / 1000_000,
            record.level(),
            record.args(),
            location.module_path(),
            location.file(),
            location.line(),
        )
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();

}

fn start_server() {
    let server = server::Server::new();
    server.run();
}
