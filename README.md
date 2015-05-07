# Example of a "flat" website with tiny-http

[![Build Status](https://travis-ci.org/tomaka/example-tiny-http.svg?branch=master)](https://travis-ci.org/tomaka/example-tiny-http)

This is an example of a website written in Rust that doesn't use callbacks.
Uses tiny-http, mustache, postgresql, and some other utility libraries.

## Heroku

The site is currently deployed at [https://example-tiny-http.herokuapp.com/](https://example-tiny-http.herokuapp.com/).
It is autodeployed by travis every time a build succeeds (see the `.travis.yml` file).

You must set the following config variables in Heroku for it to work:

```
BUILDPACK_URL=https://github.com/ddollar/heroku-buildpack-multi.git
DATABASE=<url to postgresql database>
```
