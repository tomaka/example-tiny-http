use postgres::Transaction;
use routes::authenticate;
use route_recognizer;
use templates;
use tiny_http;
use url::form_urlencoded;

use std::error::Error;
use std::io::Read;

/// GET /users/panic-example
///
/// Example of a handler that panics, to show panic handling.
pub fn handle_panic_example(_: &mut tiny_http::Request, _: &route_recognizer::Params,
                            _: &templates::TemplatesCache, _: &Transaction)
                            -> Result<tiny_http::ResponseBox, Box<Error>>
{
    panic!("Oops!");
}

/// GET /users
pub fn handle_users_list(_: &mut tiny_http::Request, _: &route_recognizer::Params,
                         templates: &templates::TemplatesCache, db: &Transaction)
                         -> Result<tiny_http::ResponseBox, Box<Error>>
{
    let users_list = db.prepare_cached("SELECT login FROM users").unwrap();
    let users_list = users_list.query(&[]).unwrap();

    let template = templates.start("users-list").unwrap();
    let template = template.insert_vec("users", |mut builder| {
        for user in &users_list {
            builder = builder.push_map(|builder| {
                let email: String = user.get("login");
                builder.insert_str("email", email)
            })
        }

        builder
    });

    Ok(template.build().boxed())
}

/// GET /users/register
pub fn handle_user_register_get(_: &mut tiny_http::Request, _: &route_recognizer::Params,
                                templates: &templates::TemplatesCache, _: &Transaction)
                                -> Result<tiny_http::ResponseBox, Box<Error>>
{
    Ok(templates.start("user-register").unwrap().build().boxed())
}

/// POST /users/register
pub fn handle_user_register_post(request: &mut tiny_http::Request, _: &route_recognizer::Params,
                                 templates: &templates::TemplatesCache, db: &Transaction)
                                 -> Result<tiny_http::ResponseBox, Box<Error>>
{
    let mut data = Vec::new();
    try!(request.as_reader().read_to_end(&mut data));
    let data = form_urlencoded::parse(&data);

    let email = data.iter().find(|&&(ref field, _)| field == "email")
                           .map(|&(_, ref val)| val.clone());
    let email = match email {
        Some(val) => val,
        None => panic!("Not email field")       // FIXME: 
    };

    let password = data.iter().find(|&&(ref field, _)| field == "password")
                              .map(|&(_, ref val)| val.clone());
    let password = match password {
        Some(val) => val,
        None => panic!("Not password field")       // FIXME: 
    };

    // FIXME: hash the password
    db.execute("INSERT INTO users(login, password) VALUES ($1, $2)", &[&email, &password]).unwrap();

    db.set_commit();
    Ok(templates.start("user-register-success").unwrap().insert_str("email", email).build().boxed())
}
