The async mailbox
=================

In this exercise, you will learn how to turn a small server implemented
using threads into an asynchronous server.

Copy `/solutions/threaded-mailbox` to your work directory.

1. Add async-std or tokio as a dependency
-----------------------------------------

Open `Cargo.toml` in your server and add:

    [dependencies]
    #...
    async-std = "1.6"

or

    [dependencies]
    # ...
    tokio = { version = "1", features = ["full"] }

2. Replace sync types by async types
------------------------------------

Replace all synchronous/blocking types (mostly `Thread` → `Task`,
`Mutex`, `TcpListener`) by async types.

Turn functions into async functions if necessary.

For `async-std`, the most relevant examples are:

-   TcpListener:
    <https://docs.rs/async-std/1.9.0/async_std/net/struct.TcpListener.html#examples>

-   Task: <https://docs.rs/async-std/1.9.0/async_std/task/index.html>

-   Mutex:
    <https://docs.rs/async-std/1.9.0/async_std/sync/struct.Mutex.html>

-   io:
    <https://docs.rs/async-std/1.9.0/async_std/io/index.html#bufreader-and-bufwriter>

For `tokio`, the most relevant examples are:

-   TcpListener:
    <https://docs.rs/tokio/1.1.1/tokio/net/struct.TcpListener.html#examples>

-   Task: <https://docs.rs/tokio/1.1.1/tokio/task/index.html>

-   Mutex: <https://docs.rs/tokio/1.1.1/tokio/sync/struct.Mutex.html>

-   io:
    <https://docs.rs/tokio/1.1.1/tokio/io/index.html#buffered-readers-and-writers>

Use those types and modules to replace the types from stdlib.

3. Observe the result
---------------------

What changed? What stayed the same? Describe your experience.

4. Testing
----------

You can test the server by sending:

    $ echo "PUBLISH 123231213" | nc 127.0.0.1 7878
    $ echo "RETRIEVE" | nc 127.0.0.1 7878

`PUBLISH <message>` will insert a message into the mailbox. `RETRIEVE`
will retrieve a message.

Alternatively, you can use the provided `tcp-client` example:

    $ cargo run -- "PUBLISH 12345"
    $ cargo run -- "RETRIEVE"

5. Additional Exercise
----------------------

-   Make the program listen on IPV4 and IPV6.

-   Implement the program using `smol` and friends instead of the above
