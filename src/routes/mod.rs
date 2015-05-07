use postgres::Transaction;
use route_recognizer;
use templates;
use tiny_http;

use std::error::Error;

mod authenticate;
mod users;
mod www;

type Handler = fn(&mut tiny_http::Request, &route_recognizer::Params, &templates::TemplatesCache,
                  &Transaction)
                  -> Result<tiny_http::ResponseBox, Box<Error>>;

// TODO: multiple individual routers would be better than a Vec (one for get, one for post, ...)
//       but all my attempts trigger an ICE
pub struct Router {
    routes: Vec<(tiny_http::Method, route_recognizer::Router<Handler>)>,
}

impl Router {
    pub fn new() -> Router {
        let mut get = route_recognizer::Router::new();
        let mut post = route_recognizer::Router::new();

        get.add("/", www::handle_home_page as Handler);

        get.add("/users", users::handle_users_list as Handler);
        get.add("/users/panic-example", users::handle_panic_example as Handler);
        get.add("/users/register", users::handle_user_register_get as Handler);
        post.add("/users/register", users::handle_user_register_post as Handler);

        Router {
            routes: vec![
                ("GET".parse().unwrap(), get),
                ("POST".parse().unwrap(), post)
            ],
        }
    }

    pub fn handle(&self, request: &mut tiny_http::Request, templates: &templates::TemplatesCache,
                  database: &Transaction)
                  -> Option<tiny_http::ResponseBox>
    {
        self.routes.iter().find(|&&(ref method, _)| method == request.get_method())
                   .and_then(|&(_, ref routes)| {
                       if let Ok(res) = routes.recognize(request.get_url()) {
                           Some((res.handler)(request, &res.params, templates,
                                              database).unwrap())
                       } else {
                           None
                       }
                   })
    }
}
