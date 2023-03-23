# Async Await

Welcome
----

-   Own your experience

-   Focus on understanding, not on the finish line

-   Feel free to code your own

-   Ask questions

Warning
----

async-await was stabilized yesterday \* Use newest nightly! \* Adjust
your expectation

Agenda
----

!
=

-   Intro (30 min)

-   Coding (20 min)

-   Programming (20min)

-   Tasks and communication (20min)

-   Pause (10min)

-   Programming (30min)

-   Shutdown and faulty connections (20min)

-   Programming (30min)

-   Closing (5min)

async-std
----

-   async-std is an implementation Rust’s async/await story

-   new library, built from the ground up

-   familiar interface, new internals

Tutorial
----

-   Follow the tutorial in <https://book.async.rs>

-   Source + Client available at:

-   <https://github.com/async-rs/a-chat>

-   Use the client to test your server

What are Futures?
----

```rust,ignore
    pub trait Future {
        type Output;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    }
```

Futures
----

-   Types that provide values… at some time in the future

-   A form of deferred computation

-   Need an executor to poll them

-   Polling is not a user concern!

async-await functions and Futures
----

----
```rust,ignore
    use async_std::io;

    async fn read_from_stdin() -> io::Result<()> {
        let stdin = io::stdin();

        // Read a line from the standard input and display it.
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        dbg!(line);

        Ok(())
    }
```

async-await functions and Futures
----

```rust,ignore
    use std::time::Duration;

    async fn timeout_on_stdin(duration: Duration) -> io::Result<()> {
        io::timeout(duration, read_from_stdin())
    }
```

Async blocks
----

-   Async blocks are blocks that get transformed into futures

-   `await` can only be called in async contexts

!
=

```rust,ignore
    use async_std::io;
    use std::time::Duration;

    async fn timeout_on_stdin(duration: Duration) -> io::Result<()> {
        io::timeout(duration, async {
            let stdin = io::stdin();

            // Read a line from the standard input and display it.
            let mut line = String::new();
            stdin.read_line(&mut line).await?;
            dbg!(line);

            Ok(())
        })
    }
```

Cold Futures
----

-   Do nothing unless polled

-   Calling an async function does nothing

Tasks
----
```rust,ignore
    use std::time::Duration;

    use async_std::io;
    use async_std::task;

    // ... other function definitions

    fn main() -> io::Result<()> {
        let duration = Duration::from_secs(5);
        // This async scope times out after 5 seconds.
        task::block_on(timeout_on_stdin(duration))
    }
```
Tasks
----

-   Tasks in async-std are similar to threads

-   They give you a `JoinHandle` and a way to retrieve their results

Executors
----

-   Components executing Futures

-   Multiple implementations possible

Exercise one
----

<https://book.async.rs>

-   3.1

-   3.2

-   3.3

Communication between Tasks
----

Channels and streams
----

-   Channels are a way to communicate between concurrent Tasks

-   Channels have a sender and a receiver

-   They are abstracted behind streams

-   Channels are closed on drop

Select/Join
----

-   `Selecting` is waiting for the first of multiple futures

-   `Joining` is waiting for all of them

Exercise Two
----

<https://book.async.rs>

-   3.4

-   3.5

-   3.6

Sleeping/Waking
----

-   Tasks are very good if they *mostly sleep*

-   They will be woken up when needed

-   New data from a client

-   Messages in a channel

Client handling
----

-   Every client `spawns` a task on connection

-   On disconnection, they need to be *reaped*

-   Disconnection might be by error

Shutdown
----

-   Shutdown needs to properly clean up all connections

Exercise Three
----

<https://book.async.rs>

-   3.7

-   3.8

-   3.9
