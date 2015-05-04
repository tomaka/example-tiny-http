use database;
use tiny_http;

/// Attempts to authenticate the user with the database.
pub fn authenticate_user(rq: &tiny_http::Request, db: &database::Transaction) -> Option<i32> {
    None
}
