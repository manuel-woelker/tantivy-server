#[macro_use]
extern crate log;

extern crate env_logger;

extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate hyper;
extern crate time;

extern crate tantivy;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

pub mod start;
pub mod server;
pub mod api;



fn main() {
    start::start();
}
