use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use time;
use tantivy;

use ::server;
use errors::*;

pub fn start() {
    if let Err(ref e) = start_internal() {
        use error_chain::ChainedError; // trait which holds `display`

        error!("{}", e.display());
        ::std::process::exit(1);
    }
}

fn start_internal() -> Result<()> {
    init_logging()?;
    info!("Starting up (Version: {}, tantivy version: {})", env!("CARGO_PKG_VERSION"), tantivy::version());
    start_server()?;
    Ok(())
}

fn init_logging() -> Result<()> {
    let format = |record: &LogRecord| {
        let t = time::now_utc();
        let location = record.location();
        format!(
            "{}.{:03} {:4.4} {} [{:10.10}|{:.15}:{}]",
            time::strftime("%Y-%m-%d %H:%M:%S", &t).expect("strftime"),
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
        builder.parse(&env::var("RUST_LOG")?);
    }

    builder.init()?;
    Ok(())
}

fn start_server() -> Result<()> {
    let server = server::Server::new();
    server.run()?;
    Ok(())
}
