In this exercise you will learn how to

-   use a web server with [actix](https://github.com/actix/actix) &
    [`Actors`](https://docs.rs/actix/0.12.0/actix/trait.Actor.html)

-   use
    [websockets](https://github.com/actix/actix-web/tree/master/actix-web-actors)
    to communicate messages between frontend and web server

-   use the [`env_logger`](https://docs.rs/env_logger/0.9.0/env_logger/)
    crate

-   implement custom message handler(s)

The web server runs at <http://localhost:8080>. Once started, open the
link in a browser. The rudimentary UI allows you to connect to the web
server, meaning you will join the chat server. Once connected you can
write messages in the text input field. Received messages will be
displayed in the text box.

Tasks
=====

1.  Broadcast the client message to all other connected clients

    -   to see the effect connect at least twice in separate tabs

2.  Implement a custom message to disconnect from the chat server

    -   you should not receive new messages from other clients
        thereafter

    -   notify other clients of disconnect

3.  Check that you do not receive your own chat message back

4.  Add heartbeat logic to remove a client from the chat server after
    10s of inactivity (e.g. tab closed)

    -   keep a time in `WsChatSession` to compare timeout with

    -   the
        [`actix::AsyncContext::run_interval`](https://docs.rs/actix/0.10.0/actix/trait.AsyncContext.html#method.run_interval)
        method is used to run a background job to check if client has
        timed out

    -   see
        [`std::time::Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html)
        and
        [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html)
        to determine the time difference

    -   use
        [`WebsocketContext::ping`](https://docs.rs/actix-web-actors/1.0.0/actix_web_actors/ws/struct.WebsocketContext.html)
        to ping client

Getting Started
===============

Clone the repository at
[ferrous-systems/teaching-material](https://github.com/ferrous-systems/teaching-material).
You find the assignment in folder `assignments/actix/chat-websockets`.

Run the web server with `cargo run` to start the chat server.

The given dependencies of the `Cargo.toml` are not the most recent
versions due to some incompatibilities. Updating these may not compile
the program anymore.

Help
====

Further documentation

-   [actix-web-actors](https://docs.rs/actix-web-actors/3.0.0/actix_web_actors/)
    API documentation, check `Message` enum

-   [actix::Recipient\#do\_send](https://docs.rs/actix/0.10.0/actix/struct.Recipient.html#method.do_send)
    API documentation to send a message to an Actor

-   [actix::ASyncContext](https://docs.rs/actix/0.10.0/actix/trait.AsyncContext.html#method.run_interval)
    documentation

Bonus Tasks
===========

1.  Add support to enter & display the client name

    -   For this you need to adapt the `index.html` page, remove the
        `visibility:hidden;` style from the `name` input field

    -   Think about how to distinguish setting the name from sending a
        text, e.g. using a prefix

    -   Expand the `WsChatSession` struct to store the name, adapt code
        accordingly

    -   If the name is available output it instead of displaying the
        text `"Someone"` in code

2.  What issues might the chat server have?

    -   What is not working as you would expect?
