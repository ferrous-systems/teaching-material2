In this exercise we will convert our existing FizzBuzz app to or create
a new FizzBizz application to command line application. Our application
will take input from command line by the user as a space separated list
of numbers.

Specification
=============

We want to create a simple command line app with the following CLI API -

    fizzbuzzcli 1 2 45 234
    fizzbuzzcli 22 1 43

You will learn to:

-   Do command line argument parsing and usage in two ways

-   Use `std::env` for basic argument parsing

-   Add external crates/libraries as dependencies

-   Use an external library/crate
    ([clap](https://docs.rs/clap/latest/clap/)) to make our lives easier

Tasks
=====

Step 1
------

Create a new binary project and name it `fizzbuzzcli`.

    cargo new --bin fizzbuzzcli

Step 2
------

Create a separate function in `main.rs` that does FizzBuzz for a given
number passed to it. You could copy it from a previous example, but try
to write it out again from scratch. The signature of the function is -

    fn fizzbuzz(num: u32) -> String

    fn fizzbuzz(num: u32) -> String {
        if num % 3 == 0 && num % 5 == 0 {
            format!("FizzBuzz")
        } else if num % 3 == 0 {
            format!("Fizz")
        } else if num % 5 == 0 {
            format!("Buzz")
        } else {
            format!("{}", num)
        }
    }

Step 3(a)
---------

Using `std::env::args()`, get the commandline arguments and save them in
a `Vec<String>`.

    // import std::env
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();
    }

Step 3(b)
---------

Iterate over the arguments, calling the `fizzbuzz` function. Pay close
attention to the first argument in the Vector of args.

Notes:

-   Parse the incoming arg as an integer, and return an integer parsing
    error if parsing fails for any element.

-   The program should run for all valid args, up to the first invalid
    one.

-   Use the `parse()` method on the incoming string. What is the return
    type of parse?

-   Then loop over those args and print the correct string for the value

<!-- -->

    // import std::env
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();
        for arg in args[1..].iter() {
            let num_from_arg = arg.parse::<u32>();
            let res = match num_from_arg {
                Ok(num) => fizzbuzz(num),
                Err(e) => format!("Error {}", e),
            };
            println!("{}", res);
        }
    }

Step 3(c)
---------

Discuss the Error handling in this solution. Discuss what needs to be
done in this solution to create a `--help` option.

Step 4(a)
---------

We will now delete the code inside `main.rs` and remove the `use`
statement for `std::env`. Here we will include an external crate/library
called `clap` into our dependencies.

In your project’s `Cargo.toml`, add `clap` as dependency. Clap’s
documentation is at <https://docs.rs/clap/latest/clap/>.

    [package]
    name = "fizzbuzzcli"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    clap = { version = "3.2.8", features = ["derive"] }

Step 4(b)
---------

Add a Struct called `Args` (you can name it anything, just be consistent
later in the code). This Struct will be what Clap can modify to "hold"
your arguments.

Notes:

-   Since, this CLI application only has a list of numbers as arguments,
    the Struct `Args` can include a member `numbers` which is a Vector.

-   Read the Clap documentation to learn which attributes to add on your
    Struct.

<!-- -->

    #[derive(Parser, Debug, Default)]
    struct Args {
        numbers: Vec<u32>,
    }

Step 4(c)
---------

Write the code in `main()` function to get the arguments and call
`fizzbuzz` on each of those.

    fn main() {
        let args = Args::parse().numbers;
        for arg in args {
            let res = fizzbuzz(arg);
            println!("{}", res);
        }
    }

### Step 4(d)

Discuss how your code changed from the previous solution.
