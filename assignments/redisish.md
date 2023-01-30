In this exercise, we will implement a toy protocol parser for a simple,
redis-like protocol.

You will learn:

-   How to write a simple Rust library

-   How to interact with borrowed and owned memory, especially how to
    take ownership

-   How to handle cases using the `match` syntax

-   How to create a safe protocol parser in Rust manually

The library does not handle I/O.

Protocol Specification
======================

The protocol has two commands:

-   `PUBLISH <message>\n`

-   `RETRIEVE\n`

With the additional properties:

1.  Messages cannot contain newlines.

2.  A missing newline at the end of the message is an error.

3.  Data after the first newline is an error.

4.  Empty messages are allowed. In this case, the message is
    `PUBLISH \n`.

Task
====

1.  Create a library project called `redisish`.

2.  Read the documentation for `str` (primitive), especially
    `split_once` and `trim`. Pay attention to their return type. Use the
    result value of `split_once` to guide your logic.

3.  Implement the following function so that it implements the rules
    above.

        pub fn parse(input: &str) -> Result<Command, Error> {
            todo!()
        }

4.  Along the way, implement appropriate datastructures for `Command`
    and `Error`.

5.  Write tests.

6.  Finishing touches: implement `std::error::Error` for your `Error`
    type. Run `clippy` on your codebase. Run `rustfmt` on your codebase.

Help
====

This section gives partial solutions to look at or refer to.

In general, we also recommend to use the Rust documentation to figure
out things you are missing to familiarise yourself with it. If you ever
feel completely stuck or that you haven’t understood something, please
hail the trainers quickly.

Getting Started
---------------

Create a new Cargo project, check the build and the test setup:

$ cargo new --lib redisish $ cd redisish $ cargo build $ cargo
test&lt;/programlisting&gt;

Appropriate data structures
---------------------------

The appropriate data structure kinds for `Command` and `Error` are:

    pub enum Command {
        //....
    }

    pub enum Error {
        //....
    }

The reason is that both describe can take multiple cases, which can be
listed. Especially the `Command` type benefits a lot by encoding that
only `Publish` carries data.

Full Data definition
--------------------

Here’s a full definition:

    #[derive(Eq,PartialEq,Debug)] 
    pub enum Command { 
        Publish(String),
        Retrieve, 
    }

    #[derive(Eq, PartialEq, Debug)]
    pub enum Error { 
        UnknownVerb,
        UnexpectedPayload,
        MissingPayload,
        EmptyMessage,
        IncompleteMessage,
    }

-   This enables comparison between 2 instances of the type, by
    comparing every field/variant. This enables the `assert_eq!` macro,
    which relies on equality being defined. `Eq` for total equality
    isn’t strictly necessary for this example, but it is good practice
    to derive it if it applies.

-   This enables automatic debug output for the type. The `assert_eq!`
    macro requires this for testing.

-   `Command` has 2 variants for the two possible commands. One carries
    data (the message), the other not.

-   The trailing comma here is optional, but considered good style.

-   `Error` is just a list of error *kinds*. This could be expanded by
    carrying more information, e.g. where the error was found in the
    input.

Dealing with borrowed data
--------------------------

If you run into the case where Rust doesn’t let you return an `&str`,
heap allocate it, by turning it into a `String`.

    pub fn parse(input: &str) -> Result<Command, Error> {
        let message: &str = { 
            //  a lot of parsing happening here
        };

        let owned_message: String = message.to_string();
    }

-   Type annotations for clarity.

-   Don’t use this pattern in your implementation, it’s here as a brief
    filler.

Testing
-------

If you struggle finding a good testing pattern or you feel like your
tests are verbose, use this pattern:

    use redisish::{self, Command, Error}; 

    #[test] 
    fn test_publish() {
        let line = "PUBLISH TestMessage\n"; 
        let result: Result<Command, Error> = redisish::parse(line); 
        let expected = Ok(Command::Publish("TestMessage".into())); 
        assert_eq!(result, expected);
    }

-   Import the types we are testing. `self` imports the `redisish`
    module, allowing us to call the very generic name `parse` as
    `redisish::parse` giving it a better visual footprint.

-   Functions marked with the `#[test]` attribute are picked up by the
    compiler and all run. If a test function *panics*, the test is
    considered failed.

-   The message under test, as a *borrowed* value. (string literals are
    always borrowed from the data section of your program)

-   Parse the borrowed value. Type annotations on the *binding* not
    required, but there for clarity.

-   Construct a value that matches the expected result.

-   `Command::Publish` requires an *owned String*, and `into()` is one
    way of getting one.

Figuring out the passed command
-------------------------------

If you need a nice pattern for figuring out what the passed command was,
use the ability to `match` on strings:

    let split = data.split_once(' ');

    match split {
        Some(("RETRIEVE", payload)) => {
             // retrieve case
        }
        Some((_,_)) {
            Err(Error::UnknownVerb)
        }
        None => {
            // maybe PUBLISH case? check verb
        }
        _ => { Err(Error::UnknownVerb) }
    }

### Full source code

If all else fails, feel free to copy this solution to play around with
it.

    use std::fmt;

    #[derive(Eq,PartialEq,Debug)]
    pub enum Command {
        Publish(String),
        Retrieve
    }

    #[derive(Eq, PartialEq, Debug)]
    pub enum Error {
        UnknownVerb,
        UnexpectedPayload,
        MissingPayload,
        EmptyMessage,
        IncompleteMessage,
        TrailingData,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Error parsing command: {:?}!", self)
        }
    }

    impl std::error::Error for Error {

    }

    pub fn parse(input: &str) -> Result<Command, Error> {
        match input.split_once('\n') {
            Some(("RETRIEVE", "")) => Ok(Command::Retrieve),
            Some((data, "")) => match data.split_once(' ') {
                Some(("PUBLISH", payload)) => Ok(Command::Publish(String::from(payload.trim()))),
                Some((_, _)) => Err(Error::UnknownVerb),
                None => Err(Error::IncompleteMessage)
            },
            Some((data, _)) => Err(Error::TrailingData),
            None => Err(Error::IncompleteMessage),
        }
    }
