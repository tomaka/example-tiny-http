#![feature(catch_panic)]

extern crate tiny_http;
extern crate mustache;
extern crate openssl;
extern crate postgres;
extern crate route_recognizer;
extern crate rustc_serialize;
extern crate url;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::Arc;

use postgres::{Connection, SslMode};

mod database;
mod routes;
mod templates;

/// Main function of the library. Starts the server.
///
/// - `db_url` must be an URL suitable for postgres.
/// - `port` is the port to use for the server.
pub fn start(db_url: &str, port: u16) {
    // we wrap the server inside an `Arc` because of the restrictions of `thread::catch_panic`
    let server = Arc::new(tiny_http::ServerBuilder::new().with_port(port).build().unwrap());
    let server = Arc::new(server);

    database::migrate(&{
        let ssl = openssl::ssl::SslContext::new(openssl::ssl::SslMethod::Sslv23).unwrap();
        let ssl = SslMode::Require(ssl);
        Connection::connect(db_url, &ssl).unwrap()
    });

    let mut join_guards = Vec::new();
    for _ in (0 .. 4) {
        let db_url = db_url.to_string();
        let server = server.clone();

        join_guards.push(thread::spawn(move || {
            loop {
                let db_url = db_url.clone();
                let server = server.clone();

                thread::catch_panic(move || -> Result<(), Box<Error>> {
                    let ssl = openssl::ssl::SslContext::new(openssl::ssl::SslMethod::Sslv23).unwrap();
                    let ssl = SslMode::Require(ssl);

                    let mut connection = try!(Connection::connect(&db_url[..], &ssl));

                    let templates = templates::TemplatesCache::new();
                    let router = routes::Router::new();

                    // iterating over requests
                    for mut request in server.incoming_requests() {
                        // trying the static files
                        if let Some(response) = serve_static(&request) {
                            request.respond(response);
                            continue;
                        }

                        // creating a new database transaction
                        let transaction = if !connection.is_desynchronized() {
                            try!(connection.transaction())
                        } else {
                            connection = try!(Connection::connect(&db_url[..], &ssl));
                            connection.transaction().unwrap()
                        };

                        // trying the routes
                        if let Some(response) = router.handle(&mut request, &templates,
                                                              &transaction)
                        {
                            request.respond(response);
                            continue;
                        }

                        // 404
                        let response = templates.start("404").unwrap().build();
                        request.respond(response.with_status_code(404));
                    }

                    Ok(())

                }).ok().map(|e| e.unwrap());
            }
        }));
    }

    for g in join_guards { g.join().unwrap(); }
}

/// Tries to find a static file that matches this function's request.
///
/// Returns `None` if it doesn't find any.
fn serve_static(request: &tiny_http::Request) -> Option<tiny_http::ResponseBox> {
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

    let content_type: &[u8] = match path.extension().and_then(|s| s.to_str()) {
        Some("gif") => b"image/gif",
        Some("jpg") | Some("jpeg") => b"image/jpeg",
        Some("png") => b"image/png",
        Some("pdf") => b"application/pdf",
        Some("htm") | Some("html") => b"text/html; charset=utf8",
        Some("txt") => b"text/plain; charset=utf8",
        Some("js") => b"application/javascript",
        Some("css") => b"text/css; charset=utf8",
        _ => b"text/plain; charset=utf8"
    };

    let response = tiny_http::Response::from_file(file)
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=3600"[..]).unwrap())
                        .boxed();
    Some(response)
}
