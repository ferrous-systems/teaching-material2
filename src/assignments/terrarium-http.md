In this exercise, we will make a simple backend request and react on its
results.

You will learn:

-   How to use the `http` crates interface

-   How to use terrariums sync and async interface

Task
----

1.  Create a new Terrarium project

2.  For every POST request:

    -   Make a HTTP POST request to `https://httpbin.org/post`,
        forwarding the request body

    -   Forward the result to the client, unchanged

3.  If the request method was not POST, respond with status `405`

Helpers
----

Making backend requests
-----------------------

Backend requests are implemented by bringing
`http_guest::{Request,RequestExt}` in scope.

`RequestExt` enables the `send` and `send_async` methods.

    use http_guest::{Request, Response, RequestExt};

    let backend_req = Request::builder()
          .method("POST")
          .uri(format!("https://httpbin.org/post"))
          .header("accept", "application/json")
          .body(req.body().clone())
          .unwrap();

    let pending = backend_req.send_async().unwrap();

Matching on the request method
------------------------------

In Rust 2015, references need to be dereferenced on match. Use the
following pattern:

    match *req.method() {
      http::Method::POST => {
          // ...
      }
      _ => {
          // ...
      }
    }

Request and Response builders
-----------------------------

`Request` and `Response` are build using the builder pattern. The
building process ends when `body` is called. Example:

    let backend_req = Request::builder()
        .method("POST")
        .uri(format!("https://httpbin.org/post"))
        .header("accept", "application/json")
        .body("Test".into())
        .unwrap();
