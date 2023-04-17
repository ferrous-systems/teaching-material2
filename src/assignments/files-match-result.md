# Files, Match and Result

In this exercise you will complete a number of mini exercises to learn about
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

Clone the teaching material repository at
[github.com/ferrous-systems/teaching-material](https://github.com/ferrous-systems/teaching-material).

[todo!] add correct location once it's definite.

1. Fix the runtime error in the template code by correcting the file path. Handle the `Result` type that is returned from the `File::open()` with a match statement, so that the `.unwrap()` can be deleted. 

2. Read the content of the file to a buffer using [Read::read\_to\_string](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string). Propagate the Error with `?` to `fn main()`. Start with the code of Step 1. 
  
3. Use the [`lines()`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines)-method to read the lines into a [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html) and count them! Start with the code of Step 1.


4. Filter out empty lines using [is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty) and print the non-empty ones. Start with the code of Step 3.

5. Write a function that parses each line and returns `Some(url)` if the line is a URL, and `None` if it is not. Use the Url crate's [UrlType](https://docs.rs/url/2.1.1/url/). Start with the code of Step 4.

## Knowledge

### Option and Result

Both `Option` and `Result` are similar in a way. Both have two
variants, and depending on what those variants are, the program may
continue in a different way.

The Option type can have the variant `Some(T)` or `None`. `T` is a type parameter that means some type should go here, we'll decide which one later.. It is used, when you have to handle optional values. For example
if you want to be able to leave a field of a struct empty, you assign the
`Option` type to it. If the field has a value, it is `Some(<value>)`, if
it is empty, it is `None`.

The variants of the `Result` type are `Ok(t)` and `Err(e)`. It is used to
handle errors. If an operation was successful, `Ok(t)` is returned. In
`Ok(t)`, `t` can be the empty tuple or a return value. In `Err(e)`, `e`
contains an error message that can be printed.

Both types can be used with the `match` keyword. The received value is
matched on patterns, each leads to the execution of a different
expression.

### How to use `match`

`match` is a way of control flow based on pattern matching. A pattern on
the left results in the expression on the right side.

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Other then with if/else, every case has to be handled explicitly, at
least with a last catch all arm that uses a place holder:

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}
```

There are different ways to use match:

The return values of the expression can be bound to a variable:

```rust
let return_value = match VALUE {
    // match arms that return a value or panic
}
```

In case of a `Result<T, E>`, match statements can be used to get to
the inner value.

```rust
let some_result: Result<File, Error> = do_operation();
match some_result {
    Ok(file) => {
        // In this block we have a new variable `file`, of type `File`
    }
    Err(error) => {
        // In this block we have a new variable `err`, of type `Error`
    }
}
```
All arms of the match tree have to result in the same type!

# Template

Clone the teaching material repository at
[github.com/ferrous-systems/teaching-material2](https://github.com/ferrous-systems/teaching-material2).

<!-- [todo!] add correct location once it's definite. -->

Then, start your `VSCode` in the proper root folder to have
`Rust-Analyzer` working properly.

```
$ teaching-material2/aassignments/_templates/files-match-result/
```

The template builds, but has a runtime error, as the location of the file
is wrong. This is intentional.

Your code will use the example data found in

```
$ teaching-material2/assignments/_templates/files-match-result/src/data
```
## Step-by-Step Solution

### Step 1: Handle the `Result` instead of unwrapping it!

`File::open` yields a `Result<T, E>` kind of type, a quick way to get to
inner type T is to use the `.unwrap()` method on the `Result<T, E>`. The
cost is that the program panics if the Error variant occurs and the
Error can not be propagated. It should only be used when the error does
not need to be propagated and would result in a panic anyways. It’s
often used as a quick fix before implementing proper error handling.

✅ Check the documentation for the exact type
    [File::open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)
    returns.

✅ Handle the `Result` using `match` to get to the inner type. Link the two possible patterns, `Ok(file)` and `Err(e)` to an an appropriate expression, for example: `println!("File opened")` and `panic!("Problem opening the file: {:?}", e)`.

✅ Fix the location of the file so that the program no longer panics. 

<details>
  <summary>Click me</summary>

```rust
fn main() {
    let open_result = File::open("src/lib/content.txt");

    match open_result {
        Ok(_file) => println!("File opened"),
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}
```

</details>

TIP: IDEs often provide a "quick fix" to roll out all match arms quickly

### Step 2: Reading the File content to a String and Error propagation.

✅ Import `std::io::prelude::*`

Take a look at [Read::read\_to\_string](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string). The method takes in a mutable empty `String`, and writes the content of a file to this buffer. The method returns a `Result<usize, Error>`, where the usize is the number of bytes that have been written to the buffer. Handling this Result, will not yield the `String` of file content. For a simple program, handling it with an 
`.unwrap()` would be sufficient, but for bigger code bases this is not helpful, so we want to propagate the Error.

✅ Add `Result<(), Error>` as return type to `fn main()` and `Ok(())` as the last line of `fn main()`.
  

✅ Create an empty `String` that serves as buffer and bind it to a mutable variable. 

✅ Call the `.read_to_string()` method on the `File` object. The method takes in the `String` buffer and is followed by the `?` operator. If the method returns an `Error` it is propagated to the instance that called `fn main()`. If the method returns the `Ok` value, the program proceeds as planned.

✅ Use `println!` to print the content of the `String` buffer.

<details>
  <summary>Click me</summary>

```rust
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let open_result = File::open("src/data/content.txt");

    let mut file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut content_string = String::new();
    file.read_to_string(&mut content_string)?;

    println!("{}", content_string);
    Ok(())
}
```
</details>

### Task 3: Read the lines into a `BufReader` and count them!

✅ Add the following imports:

```rust
use std::io::{ BufReader, BufRead,}
```

✅ Take a look at the documentation of [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html). BufReader is a struct that adds buffering to any reader. It implements the
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html#) trait. In short this means, that methods that are defined for `BufRead` can be used for `BufReader`. For example the [`lines()`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines) method.

✅ Construct a `BufReader` around the file.

✅ The `lines()`- method returns an Iterator over the file’s lines. Iterate over the lines with a for loop to count them.

✅ Print the number of lines the file contains.

✅ You don’t have to handle the `Result` that is returned from `.lines()`, why?

<details>
  <summary>Click me</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let open_result = File::open("./src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    let mut number = 0;

    for _line in buf_reader.lines() {
        number += 1;
    }

    println!("{}", number);
}
```

</details>

### Step 4: Filter out empty lines print the Rest …and Error Handling

✅ Filter out the empty lines, and only print the the others. The [is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty) method can help you here.

<details>
  <summary>Click me</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        match line {
            Ok(content) => {
                if !content.is_empty() {
                    println!("{}", content)
                }
            }
            Err(e) => println!("Error reading line {}", e),
        }
    }
}
```

</details>

### Step 5: Read URLs from file and return with Option.

✅ Add `url = "2"` to your `[dependencies]` section in `Cargo.toml` and import `url::Url` in `main.rs`.

✅ Write a function that parses each line using the [UrlType](https://docs.rs/url/2.1.1/url/). Search the docs for a method for this!

```rust
fn parse_url(line: String) -> Option<Url> {
    todo!
}
```

✅ If a line can be parsed successfully, return `Some(url)`, `None` otherwise.

✅ In the calling context, only print URLs that parse correctly.

✅ Test the `fn parse_url()`.

<details>
  <summary>Click me</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use url::Url;

fn parse_line(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let line = match line {
            Ok(content) => content,

            Err(e) => panic!("Error reading line {}", e),
        };

        let url = parse_line(line);

        match url {
            Some(line) => println!("{}", line),
            None => continue,
        }
    }
}

#[test]
fn correct_url() {
    assert!(parse_line(String::from("https://example.com")).is_some())
}

#[test]
fn no_url() {
    assert!(parse_line(String::from("abcdf")).is_none())
}
```

</details>

## Help

### Typing variables

Variables can be typed by using `:` and a type.

    let my_value: String = String::from("test");


