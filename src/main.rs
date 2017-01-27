extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate hyper;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use iron::prelude::*;
use iron::status;
use std::path::Path;

use iron::headers::{ContentType};
use iron::response::{WriteBody, ResponseBody};
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use mount::Mount;
use staticfile::Static;
use serde::Serialize;
use iron::modifier::Modifier;

#[derive(Debug, Serialize, Deserialize)]
struct StatusResponse {
    version: String,
    health: String,
}

struct Json<T: Serialize>(T);

impl <T: Serialize + Send + 'static> Modifier<Response> for Json<T> {
    fn modify(self, resp: &mut Response) {
        resp.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                                 vec![(Attr::Charset, Value::Utf8)])));
        resp.body = Some(Box::new(self));
    }
}
fn json_to_io_error(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e)
}

impl <T: Serialize + Send> WriteBody for Json<T> {
    fn write_body(&mut self, res: &mut ResponseBody) -> std::io::Result<()> {
        serde_json::ser::to_writer_pretty(res, &self.0).map_err(json_to_io_error)?;
        Ok(())
    }
}

fn main() {
    let status_response = StatusResponse {version: env!("CARGO_PKG_VERSION").into(), health: "octarine".into()};
    println!("Status: {:?}", status_response);
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
/*        Ok(Response::with((status::Ok, Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                                         vec![(Attr::Charset, Value::Utf8)]))), format!("{{\"status\": \"green\", \"version\": \"{}\"}}", env!("CARGO_PKG_VERSION")))))
                                                         */
        let status_response = StatusResponse {version: env!("CARGO_PKG_VERSION").into(), health: "octarine".into()};
        Ok(Response::with((status::Ok, Json(status_response))))

    });

    let listening = Iron::new(mount).http("localhost:3000");
    let listening = listening.unwrap();
    println!("Server started on http://localhost:{}/", port);
    println!("{:?}", listening);
}