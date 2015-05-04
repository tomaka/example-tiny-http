#![feature(catch_panic)]

extern crate tiny_http;
extern crate mustache;
extern crate openssl;
extern crate postgres;
extern crate route_recognizer;
extern crate rustc_serialize;

use std::any::Any;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::sync::Arc;

mod database;
mod routes;
mod templates;

/// Main function of the library. Starts the server.
///
/// - `db_url` must be an URL suitable for postgres.
/// - `port` is the port to use for the server.
pub fn start(db_url: &'static str, port: u16) {
    // we wrap the server inside an `Arc` because of the restrictions of `thread::catch_panic`
    let server = Arc::new(tiny_http::ServerBuilder::new().with_port(port).build().unwrap());
    let server = Arc::new(server);

    loop {
        let server = server.clone();
        thread::catch_panic(move || -> Result<(), Box<Error>> {
            // unfortunately the database connection can't be put in an `Arc`
            let pool = try!(database::ConnectionPool::new(db_url));
            let templates = templates::TemplatesCache::new();
            let router = routes::Router::new();

            // iterating over requests
            for mut request in server.incoming_requests() {
                // trying the static files
                if let Some(response) = serve_static(&request) {
                    request.respond(response);

                // trying the routes
                } else if let Some(response) = router.handle(&mut request, &templates,
                                                             &pool.transaction())
                {
                    request.respond(response);

                // 404
                } else {
                    let response = templates.start("404").unwrap().build();
                    request.respond(response.with_status_code(404));
                }
            }

            Ok(())

        }).ok().map(|e| e.unwrap());
    }
}

/// Tries to find a static file that matches this function's request.
///
/// Returns `None` if it doesn't find any.
fn serve_static(request: &tiny_http::Request) -> Option<tiny_http::Response<Box<Read + Send>>> {
    let path = Path::new("./src/static").join(Path::new(&request.get_url()[1..]));

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None
    };

    match file.metadata() {
        Err(_) => return None,
        Ok(ref metadata) if metadata.is_dir() => return None,
        Ok(_) => ()
    };

    // FIXME: security, handle '..'

    let content_type = match path.extension().and_then(|s| s.to_str()) {
        Some("gif") => "image/gif",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("pdf") => "application/pdf",
        Some("htm") | Some("html") => "text/html; charset=utf8",
        Some("txt") => "text/plain; charset=utf8",
        Some("js") => "application/javascript",
        Some("css") => "text/css; charset=utf8",
        _ => "text/plain; charset=utf8"
    };

    let response = tiny_http::Response::from_file(file)
                        .with_header(format!("Content-Type: {}", content_type).parse().unwrap())
                        .boxed();
    Some(response)
}