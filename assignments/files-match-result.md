# Files, Match and Result

In this exercise you will complete a number of steps to learn about
Error Handling. The final result will be a url parser that reads lines
from a text file and can distinguish the content between URLs and
non-urls.

## In this exercise, you will learn how to

-   handle occurring `Result`-types with `match` for basic error
    handling.

-   when to use the `.unwrap()` method.

-   propagate an error with the `?` operator

-   return the `Option`-type.

-   do some elementary file processing (opening, reading to buffer,
    counting, reading line by line).

-   navigate the Rust `stdlib` documentation

-   add external dependencies to your project

## Task

-   Manually unwrap the `Result` type that is returned from the
    File::open() with a match statement, so that the .unwrap() can be
    deleted.

-   Move this manual unwrap to it’s own function.

-   Read the content of the file to a buffer and count the lines in a
    for loop.

-   Filter out empty lines and print the non-empty ones.

-   Write a function that parses each line and returns Some(url) if the
    line is a URL, and `None` if it is not. Use the Url crate [Url
    Type](https://docs.rs/url/2.1.1/url/)

## Knowledge

### Option and Result

Both the `Option` and `Result` are similar in way. Both have two
variants, and depending on what those variants are, the program may
continue in a different way.

The Option Type can have the variant `Some(<some other type>)` or
`None`. It is used, when you have to handle optional values, for example
if you want to be able to leave a field of a struct empty, go assign the
option type to it. If the field has a value, it is `Some(<value>)`, if
it is empty, it is `None`.

The variants of the Result type are `Ok(t)` and `Err(e)`. It is used to
handle errors. If an operation was successful, `Ok(t)` is returned. In
`Ok(t)`, `t` can be the empty tuple or a return value. In `Err(e)`, `e`
contains an error message that can be printed.

Both types can be used with the `match` keyword. The received value is
matched on patterns, each leads to the execution of a different
expression.

### how to use `match`

`match` is a way of control flow based on pattern matching. A pattern on
the one left evaluates to an expression on the right side.

    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }

other then with if/else, every case has to be handled explicitly, at
least with a last catch all arm using a place holder:

    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        _ => EXPRESSION,
    }

There are different ways to use match:

The return values of the expression can be bound to a variable:

    let return_value = match VALUE {
        // match arms that return a value or panic
    }

In case of a Result<T, E>, match statements can be used to get to
the inner value.

    match RESULT<T,E> {
        Ok(T) => EXPRESSION, that uses or returns T
        Err(E) => EXPRESSION,
    }

All arms of the match tree have to result in the same type!

# Template

Clone the teaching material repository at
[github.com/ferrous-systems/teaching-material](https://github.com/ferrous-systems/teaching-material).

Then, start your `VSCode` in the proper root folder to have
`Rust-Analyzer` working properly.

    git clone https://github.com/ferrous-systems/teaching-material
    code teaching-material/assignments/files-match-result-assignment/template/

The template builds but has a runtime error, as the location of the file
is wrong. This is intentional.

Your code will use the example data found in
[files-match-result-assignment/template/src/data](https://github.com/ferrous-systems/teaching-material/tree/main/assignments/files-match-result-assignment/template/src/data).

## Step-by-Step Solution

### Step 1: Unwrapping

`File::open` yields a `Result<T, E>` kind of type, a quick way to get to
inner type T is to use the `.unwrap()` method on the `Result<T, E>`. The
cost is that the program panics if the Error variant occurs and the
Error can not be propagated. It should only be used when the error does
not need to be propagated and would result in a panic anyways. It’s
often used as a quick fix before implementing proper error handling.

-   Check the documentation for the exact type
    [File::open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)
    returns.

-   implement a manual unwrap using `match` so to get to the inner type.
    Link the two possible patterns, `Ok(file)` and `Err(e)` to an an
    appropriate expression, for example: `println!("File opened")` and
    `println!("Error: {}", e)`

-   fix the location of the file so that the error is no longer
    returned. TIP: IDEs often provide a "quick fix" to roll out all
    match arms quickly

### Step 2: Moving the unwrapping into a function.

-   Implement the following function based on what you wrote in the last
    step.

<!-- -->

    fn unwrap_file(open_result: Result<File, Error>) -> File {
        todo!
    }

-   change `println!("Error: {}", e)` to `panic!("Error: {}", e)`

-   to be able to return from the `Ok()` arm, add a `return` statement
    to return the `File` object.

    -   call the function.

### Step 3: Reading the File content and Error propagation.

-   import `std::io::prelude::*`

-   Take a look at
    [Read::read\_to\_string](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string).
    The method takes in a mutable empty `String`, and writes the content
    of a file to this buffer. The method returns a
    `Result<usize, Error>`, where the usize is the number of bytes that
    have been written to the buffer. Handling this Result, will not
    yield the `String` of file content. For a simple program, handling
    it with an `.unwrap()` would be sufficient, but for bigger code
    bases this is not helpful.

Instead, add the following function to your program. It takes in a
mutable instance of the `File` and returns a `Result<String, Error>`. It
creates an empty `String` that serves as buffer. The `.read_to_string()`
method is applied on the `File` object. The method takes in the `String`
buffer. The method is then followed by the `?` operator. If the method
returns an `Error` the function propagates this error. If the method
returns the `Ok` value, the funktion returns the `String` wrapped in the
`Ok`.

    fn content_to_string(mut file: File) -> Result<String, Error> {
        let mut content_string = String::new();
        file.read_to_string(&mut content_string)?;
        Ok(content_string)
    }

-   In `main()`, call the function and bind it to a variable. You can
    use `.unwrap()` to handle the `Result`.

-   Use `println!` to print the content of the `String`

### Task 4: Counting lines

-   add the following imports

<!-- -->

    use std::io::{ BufReader, BufRead,}

-   Take a look at the documentation
    [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html).
    BufReader is a struct that adds buffering to any reader. It
    implements the
    [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html#)
    trait. In short this means, that methods that are defined for
    `BufRead` can be used for `BufReader`. For example the
    [`lines()`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines)
    method.

-   Construct a `BufReader` around the file.

-   The `lines()`- method returns an Iterator over the file’s lines.
    Iterate over the lines with a for loop to count them.

-   Print the number of lines the file contains.

-   You don’t have to handle the `Result` that is returned from
    `.lines()`, why?

### Step 5: Filter out empty lines print the Rest …and Errors

-   `lines` returns the `Result`-Type, use it with a `match` statement
    to get to the actual `String`.

-   Filter out the empty lines, and only print the the others. The
    [is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty)
    method can help you here.

### Step 6: Read URLs from file and return with Option.

-   Add `url = "2"` to your `[dependencies]` section in `Cargo.toml` and
    import `url::Url` in `main.rs`.

-   Write a function that parses each line using the [Url
    Type](https://docs.rs/url/2.1.1/url/). Search the docs for a method
    for this!

<!-- -->

    fn parse_url(line: String) -> Option<Url> {
        todo!
    }

-   If a line can be parsed successfully, return `Some(url)`, `None`
    otherwise

-   In the calling context, only print URLs that parse correctly

-   Test the `parse_url`

## Help

### Typing variables

Variables can be typed by using `:` and a type.

    let my_value: String = String::from("test");
