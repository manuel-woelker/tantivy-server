extern crate iron;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use iron::status;
use std::path::Path;

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

    let listening = Iron::new(mount).http("localhost:3000");
    let listening = listening.unwrap();
    println!("Server started on http://localhost:{}/", port);
    println!("{:?}", listening);
}