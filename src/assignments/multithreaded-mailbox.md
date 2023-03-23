Bring your TCP server and the protocol we wrote yesterday together.

In this exercise, you will learn:

-   To make a single-thread codebase multithreaded

-   The basics of Rusts Thread API

-   How to use the `sync` module to allow sharing between concurrent
    units

Implement multithreading
----

Using your existing codebase, implement the following:

For every connection:

1.  spawn a thread using `std::thread::spawn`

2.  handle the client within this thread

3.  wrap the `VecDeque` in a `std::sync::Mutex` and an `std::sync::Arc`
    to pass it around

Notes
----

Base code
---------

If you want to start from a clean state, you can use the solution code
from the last exercise:
<https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/tcp-client>

`tcp-client`
------------

To send and receive messages, you can either use `nc` or `telnet`.
Alternatively, you can use the client provided:
<https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/tcp-client>

Usage:

    cargo run -- "PUBLISH This is my message"
    cargo run -- "RETRIEVE"

Help
----

Wrapping the `VecDeque` correctly
---------------------------------

When spawning a thread around our handling function, the compiler will
complain for multiple reasons: the queue in not safely shared and not
safely synchronised.

-   `Mutex` provides the synchronisation

-   `Arc` provides the safe sharing

So the correct way is to wrap the queue in a synchronisation primitive
and then share the primitive.

    let queue = Arc::new(Mutex::new(VecDeque::new()));

### `handle` signature

If you are confused about what new signature you need for `handle`, this
one is a good start:

    fn handle(stream: TcpStream, queue: Arc<Mutex<VecDeque<String>>>>) {
        ///
    }

### Correctly sharing

For sharing the `Arc` correctly, a new handle must be acquired *before*
moving it into the thread.

    for stream in listener.incoming() {
        let stream = stream?;
        let shared_queue = queue.clone();
        std::thread::spawn(move || {
            handle(stream, shared_queue);
        })
    }

### Locking the Mutex

Locking the Mutex works by calling `.lock()`. The result should be
unwrapped, the trainer will later explain why.

The lock can then be used like the normal value.

    let msg = String::from("test");
    let mut lock = queue.lock().unwrap();
    lock.push_back(msg);
