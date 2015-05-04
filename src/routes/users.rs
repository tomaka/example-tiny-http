use database;
use routes::authenticate;
use route_recognizer;
use templates;
use tiny_http;

use std::error::Error;
use std::io::Read;

/// GET /users/panic-example
///
/// Example of a handler that panics, to show panic handling.
pub fn handle_panic_example(request: &mut tiny_http::Request, _: &route_recognizer::Params,
                            templates: &templates::TemplatesCache, db: &database::Transaction)
                            -> Result<tiny_http::Response<Box<Read + Send>>, Box<Error>>
{
    panic!("Oops!");
}

/// GET /users/register
pub fn handle_user_register_get(request: &mut tiny_http::Request, _: &route_recognizer::Params,
                                templates: &templates::TemplatesCache, db: &database::Transaction)
                                -> Result<tiny_http::Response<Box<Read + Send>>, Box<Error>>
{
    Ok(templates.start("user-register").unwrap().build().boxed())
}

/// POST /users/register
pub fn handle_user_register_post(request: &mut tiny_http::Request, _: &route_recognizer::Params,
                                 templates: &templates::TemplatesCache, db: &database::Transaction)
                                 -> Result<tiny_http::Response<Box<Read + Send>>, Box<Error>>
{
    let mut data = String::new();
    try!(request.as_reader().read_to_string(&mut data));

    panic!("post");
}
