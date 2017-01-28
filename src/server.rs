
use iron::prelude::*;
use iron::status;
use std::path::Path;

use iron::headers::ContentType;
use iron::response::{WriteBody, ResponseBody};
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use mount::Mount;
use staticfile::Static;
use serde::Serialize;
use iron::modifier::Modifier;

use std;

use api;
use errors::*;

use serde_json;


pub struct JsonResponse<T: Serialize>(pub T);

impl<T: Serialize + Send + 'static> Modifier<Response> for JsonResponse<T> {
    fn modify(self, resp: &mut Response) {
        resp.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));
        status::Ok.modify(resp);
        resp.body = Some(Box::new(self));
    }
}
fn json_to_io_error(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e)
}

impl<T: Serialize + Send> WriteBody for JsonResponse<T> {
    fn write_body(&mut self, res: &mut ResponseBody) -> std::io::Result<()> {
        serde_json::ser::to_writer_pretty(res, &self.0).map_err(json_to_io_error)?;
        Ok(())
    }
}


pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(&self) -> Result<()> {
        let port = 3000;
        let mut mount = Mount::new();

        // Serve the shared JS/CSS at /
        mount.mount("/",
                    |_: &mut Request| Ok(Response::with((status::Ok, "<h1>Tantivy Server</h1>"))));
        // Serve the static file docs at /doc/
        mount.mount("/swagger/",
                    Static::new(Path::new("docs/api/api.swagger.yaml")));
        mount.mount("/docs/swagger/",
                    Static::new(Path::new("assets/swagger-ui/")));
        mount.mount("/api/status", api::status::StatusHandler {});

        let listening = Iron::new(mount).http("localhost:3000")?;
        info!("Server started on http://localhost:{}/", port);
        std::mem::drop(listening);
        Ok(())
    }
}
