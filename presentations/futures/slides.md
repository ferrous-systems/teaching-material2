[Table of Contents](./index.html)

!
=

The crate [`futures-rs`](https://github.com/alexcrichton/futures-rs) is
commonly used to build asyncronous functionality. It provides constructs
similar to \`\`Promise\`\`s in Javascript.

Notes on performance and usability
==================================

-   Rust does not have an implicit runtime event loop like Node.js.

    -   Tokio provides an explicit one.

-   Futures are a zero-cost abstraction.

-   The documentation for `futures-rs` is regarded as *not good*.

-   The async ecosystem is still very young. Be patient!

You’ve got oneshot
==================

`futures::sync::oneshot` provides a basic, single use future.

They feel like a channel to use, even coming with a `tx` and `rx` pair.

You’ve got oneshot
==================

    fn main() {
        // This is a simple future, sort of like a one-time channel.
        // You get a (sender, receiver) when you invoke them.
        // Sending a value consumes that side of the channel.
        let (tx, rx) = oneshot::channel();

        // This consumes the sender, we can't use it afterwards.
        tx.send("Bears").unwrap();

        // Now we can wait for it to finish
        let result = rx.wait()
            .unwrap();
        println!("{}", result);
    }

    use futures::Future;
    use futures::sync::oneshot;

You’ve got oneshot
==================

What happens if we swap the `rx.wait()` and the `tx.send()`?

There is **no** implicit threading, calling `rx.wait()` blocks the
thread until data is received!

You’ve got oneshot (threads)
============================

    fn main() {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            tx.send("Bears").unwrap();
        });

        let result = rx.wait()
            .unwrap();
        println!("{}", result);
    }

    use std::thread;
    use std::time::Duration;
    use futures::Future;
    use futures::sync::oneshot;

Multiple oneshots
=================

    const NUM_OF_TASKS: usize = 10;

    fn main() {
        let mut rx_set = Vec::new();

        for index in 0..NUM_OF_TASKS {
            let (tx, rx) = futures::oneshot();
            rx_set.push(rx);

            thread::spawn(move || {
                println!("{} --> START", index);
                sleep_a_little_bit();
                tx.send(index).unwrap();
                println!("{} <-- END", index);
            });
        }

        // `join_all` lets us join together the set of futures.
        let result = join_all(rx_set)
            .wait()
            .unwrap();

        println!("{:?}", result);
    }

    use std::thread;
    use futures::Future;
    use futures::future::join_all;
    use std::time::Duration;
    use rand::distributions::{Range, IndependentSample};

    // This function sleeps for a bit, then returns how long it slept.
    pub fn sleep_a_little_bit() -> u64 {
        let mut generator = rand::thread_rng();
        let possibilities = Range::new(0, 1000);

        let choice = possibilities.ind_sample(&mut generator);

        let a_little_bit = Duration::from_millis(choice);
        thread::sleep(a_little_bit);
        choice
    }

57 channels (and nothing on)
============================

An `futures::sync::mpsc` represents a channel that will yield a series
of futures.

`mpsc::channel` has a bounded buffer size, and is concerned with back
pressure.

`mpsc::unbounded` has no bounded size, and can grow to fit all of
memory.

57 channels (and nothing on)
============================

    const BUFFER_SIZE: usize = 57;

    fn main() {
        // We're using a bounded channel here with a limited size.
        let (mut tx, rx) = mpsc::channel(BUFFER_SIZE);

        thread::spawn(move || {
            for index in 0..(BUFFER_SIZE+2) {
                sleep_a_little_bit();
                // When we `send()` a value it consumes the sender. Returning
                // a 'new' sender which we have to handle. In this case we just
                // re-assign.
                match tx.send(index).wait() {
                    // Why do we need to do this? This is how back pressure is implemented.
                    // When the buffer is full `wait()` will block.
                    Ok(new_tx) => tx = new_tx,
                    Err(_) => panic!("Oh no!"),
                }
            }
            // Here the stream (`tx`) is dropped, completing it.
        });

        // We can `.fold()` like we would an iterator. In fact we can do many
        // things like we would an iterator.
        let sum = rx.fold(0, |acc, val| {
                // Notice when we run that this is happening after each item of
                // the stream resolves, like an iterator.
                println!("--- FOLDING {} INTO {}", val, acc);
                // `ok()` is a simple way to say "Yes this worked."
                // `err()` also exists.
                ok(acc + val)
            })
            .wait()
            .unwrap();
        println!("Sum: {}", sum);
    }

    use std::time::Duration;
    use std::thread;
    use rand::distributions::{Range, IndependentSample};
    use futures::future::{Future, ok};
    use futures::stream::Stream;
    use futures::sync::mpsc;
    use futures::Sink;

    // This function sleeps for a bit, then returns how long it slept.
    pub fn sleep_a_little_bit() -> u64 {
        let mut generator = rand::thread_rng();
        let possibilities = Range::new(0, 100);

        let choice = possibilities.ind_sample(&mut generator);

        let a_little_bit = Duration::from_millis(choice);
        thread::sleep(a_little_bit);
        choice
    }

Toes in the CPU pool
====================

`futures-rs` comes with
[`futures_cpupool`](https://docs.rs/futures-cpupool/0.1.7/futures_cpupool/)
which provides a simple, easy to use CPU Pool type.

This allows for us to dispatch arbitrary (heterogeneous!) jobs to a pool
without worrying about where (and when) it gets executed.

Toes in the CPU pool
====================

    fn main() {
        // Creates a CpuPool with workers equal to the cores on the machine.
        let pool = Builder::new().create();

        let returns_string = pool.spawn_fn(move || -> Result<_, ()> {
            Ok("First")
        });
        let returns_integer = pool.spawn_fn(move || -> Result<_, ()> {
            Ok(2)
        });

        let resulting_string = returns_string.wait().unwrap();
        let resulting_integer = returns_integer.wait().unwrap();

        println!("{}, {}", resulting_string, resulting_integer);
        // Returns `First, 2`
    }

    use futures::future::Future;
    use futures_cpupool::Builder;

Interacting with futures
========================

Most of the times you will not be creating raw futures and sending them
around.

Instead, you’ll likely end up interacting with them as part of a crate.

Worry more about how to handle them, and work with them, than how to
create and execute them.
