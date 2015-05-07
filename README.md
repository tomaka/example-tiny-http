# Example of a "flat" website with tiny-http

This is an example of a website written in Rust that doesn't use callbacks.

## Heroku

The site is currently deployed at [https://example-tiny-http.herokuapp.com/](https://example-tiny-http.herokuapp.com/).

You must set the following config variables:

```
BUILDPACK_URL=https://github.com/ddollar/heroku-buildpack-multi.git
DATABASE=???
```
