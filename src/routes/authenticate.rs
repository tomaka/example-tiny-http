use postgres;
use tiny_http;

/// Attempts to authenticate the user with the database.
pub fn authenticate_user(rq: &tiny_http::Request, db: &postgres::Transaction) -> Option<i32> {
    None
}
