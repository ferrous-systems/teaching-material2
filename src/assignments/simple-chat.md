Implement a simple chat using `async-std`.

In this exercise, you will learn:

-   How to use async tasks for synchronisation

-   How to do simple connection handing

-   How to use async channels

Note: this is a trimmed down version of the tutorial found in the
[async-std book](https://book.async.rs/tutorial/index.html).

1. Clone the template
----

    $ git clone git@github.com:skade/async-chat-template.git

2. Protocol Specification
----

The protocol is simple:

-   The first line a client sends is its name

-   Every subsequent line is a message

3. Task
----

Implement the client function so that it:

-   It reads the name of the client

-   Then registers the client with the broker

-   Spawns 2 tasks, one to read the incoming messages, one to deliver
    outgoing

Go step by step, first reading the name from the input, printing it
using `println` and then going from there.

Help
----

Clone everywhere
----------------

This is an unoptimised example and moves `String` around everywhere.
`clone` liberally, optimisations can come later.

Iterating over incoming lines
-----------------------------

    let mut buf_read = BufReader::new(stream.clone());

    let mut lines = buf_read.lines();

    while let Some(Ok(line)) = lines.next().await {
        //...
    }
