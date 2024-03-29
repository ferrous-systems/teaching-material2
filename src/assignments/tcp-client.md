In this exercise, we will implement a simple TCP client.

You will learn how to:

-   write a simple single-threaded client tool

-   connect to your server and send/receive strings

-   close parts of a bidirectional stream

Task
----

1.  Start a TCP connection on port `127.0.0.1:7878`

2.  Send a message over

3.  Close the write end of the connection

4.  Print the response to the console

Getting started
----

Use this template:

    use std::io;

    fn main() -> io::Result<()> {
        let arg = std::env::args().nth(1);

        let message = match arg {
            Some(msg) => msg,
            None => String::from("Default Message"),
        };


        //...
        Ok(())
    }

Read the documentation for the `TcpStream`, especially the `connect()`
and `shutdown()` methods.
