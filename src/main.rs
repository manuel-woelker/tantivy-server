extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use std::path::Path;

use iron::headers::{Headers, ContentType};
use iron::modifiers::Header;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use mount::Mount;
use staticfile::Static;

fn main() {
    let port = 3000;
    println!("Starting server on port {}", port);
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /
    mount.mount("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "<h1>Tantivy Server</h1>")))
    });
    // Serve the static file docs at /doc/
    mount.mount("/swagger/", Static::new(Path::new("docs/api/api.swagger.yaml")));
    mount.mount("/docs/swagger/", Static::new(Path::new("assets/swagger-ui/")));
    mount.mount("/api/status", |_: &mut Request| {
        let mut headers = Headers::new();

        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]))
        );
        Ok(Response::with((status::Ok, Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                                         vec![(Attr::Charset, Value::Utf8)]))), format!("{{\"status\": \"green\", \"version\": \"{}\"}}", env!("CARGO_PKG_VERSION")))))
    });

    let listening = Iron::new(mount).http("localhost:3000");
    let listening = listening.unwrap();
    println!("Server started on http://localhost:{}/", port);
    println!("{:?}", listening);
}