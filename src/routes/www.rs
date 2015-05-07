use postgres::Transaction;
use route_recognizer;
use templates;
use tiny_http;

use std::error::Error;

/// GET /
pub fn handle_home_page(_: &mut tiny_http::Request, _: &route_recognizer::Params,
                        templates: &templates::TemplatesCache, _: &Transaction)
                        -> Result<tiny_http::ResponseBox, Box<Error>>
{
    Ok(templates.start("home").unwrap().build().boxed())
}
