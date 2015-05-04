extern crate example_tiny_http;

fn main() {
    // ensures that stack traces will be printed
    std::env::set_var("RUST_BACKTRACE", "1");

    let db = std::env::var("DATABASE").unwrap_or("postgres://postgres:postgres@localhost/my_db".to_string());
    example_tiny_http::start(&db, 8000);
}
