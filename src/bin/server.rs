extern crate example_tiny_http;

fn main() {
    // ensures that stack traces will be printed
    std::env::set_var("RUST_BACKTRACE", "1");

    example_tiny_http::start("postgres://postgres:postgres@localhost/my_db", 8000);
}
