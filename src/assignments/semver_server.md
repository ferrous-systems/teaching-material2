In this exercise, we will implement a crate repository server.

You will learn:

-   How to use modules and visibility

-   How to write a simple single-threaded server binary

-   Read Strings from the network

-   How to handle a connection lifecycle

Task
----

1.  Move all your previous work on `SemVer` into a `lib.rs` module

2.  Create a skeleton TCP server based on
    <https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/tcp-echo-server>

3.  Deserialize incoming crates based on the following protocol:

    -   `GET <crate_name>\n`

    -   `PUT <crate_data_without_newlines>\n`

4.  on PUT, store the crate data in your repository.

5.  on GET, return a crate from your repository, or a custom error if it
    cannot be found.

6.  Implement `std::fmt::Display` for your custom error. Also derive
    `std::fmt::Debug`.

7.  Inspect the `std::error::Error` trait. What is needed to make your
    custom error conform to that trait? Add all that’s required.

8.  Any functions returning your Error can now be generalized to return
    `Box<dyn Error>`. Change them accordingly and simplify existing
    error handling where possible.

Note
----

To send and receive messages, you can either use `nc` or `telnet`.
Alternatively, you can use the client provided:
<https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/tcp-client>

The entire source tree can be found here:

$ git clone https://github.com/ferrous-systems/teaching-material.git $
cd teaching-material/assignments/solutions/tcp-client $ cargo run
testmessage&lt;/programlisting&gt;
