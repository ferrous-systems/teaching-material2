# Error Handling

!
=

Error handling is explicit in Rust.

Any function with known error conditions returns a `Result<T,E>`.

**There are no exceptions.**

```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn main() {
        let outcome = this_can_fail(true);
        println!("{:?}", outcome);
    }
```
Results Must Be Used
====================
```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn main() {
        this_can_fail(true);
    }
```

Using Results With `match`
==========================
```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn main() {
        match this_can_fail(false) {
            Ok(val) => println!("Success: {}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
```
Using Results With Conditionals
===============================

Check for success with `is_ok()`, errors with `is_err()`:

```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn main() {
        if this_can_fail(false).is_ok() {
            println!("It worked!");
        } else {
            println!("It didn't work!")
        }
    }
```
Using Results With `?`
======================

Use `?` in functions with multiple possible failures.

```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn multiple_possible_failures() -> Result<String,String> {
        this_can_fail(true)?;
        println!("After 1st potential error.");
        this_can_fail(false)?;
        println!("After 2nd potential error.");
        Ok(String::from("All done."))
    }

    fn main() {
        multiple_possible_failures();
    }
```
Using Results With `?`
======================

Note the early exit. Click on the `Play` button on the top right corner!

Using `?` in `main`
===================

-   `main` can return `Result`

TODO: Fix this example

```rust,ignore,does_not_compile,editable
    use std::error::Error;

    fn maybe_dangerous() -> Option<()>{
        Some(())
    }

    fn main() -> Result<(), Box<dyn Error>> {
        maybe_dangerous()?;
    }
```
Mapping Result Values
=====================

```rust,editable
    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Error"))
        }
    }

    fn main() {
        let some_result = this_can_fail(true);
        // Only done if `some_result` is an `Ok` Variant.
        let mapped_result = some_result.map(|val| val.len());
        println!("{:?}", mapped_result);
    }
```
`map_err()` is also available.

Dynamic errors
==============

Rust has an idiomatic dynamic error type, which most errors can be
converted into:

```rust,does_not_compile,ignore,editable
    use std::fs::File;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let f = File::open("broken")?;

        let x: i32 = "ABC".parse()?;

        Ok(())
    }
```
To be converted like this, errors must implement the std::error::Error
trait.

Reporting Errors Only
=====================

If you only have to report an error, but don’t have a meaningful return
value, use `Result<(), Error>`.
