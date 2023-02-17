# Crates

!
=

Rust calls libraries `crates`. Management of crates is generally done
through `cargo`, but this is not strictly necessary.

Usage of libraries
==================

In Rust 2015 you need to declare crates via the
`extern crate`-statement.

```rust,ignore
    extern crate serde_json;

    use serde_json::Value;

    fn main() {
        let data = r#" { "name": "John Doe", "age": 43, ... } "#;
        let v: Value = serde_json::from_str(data)?;
        println!(
            "Please call {} at the number {}",
            v["name"],
            v["phones"][0]
        );
    }
```
This imports the "SERialisation/DEserialisation"-Framework.

!
=

Rust 2018 onwards this is no longer required. Libraries that were
declared via cargo can simply be used with `use`-statements.
```rust,ignore
    use serde_json::{self, Value};

    fn main() {
        let data = r#" { "name": "John Doe", "age": 43, ... } "#;
        let v: Value = serde_json::from_str(data)?;
        println!(
            "Please call {} at the number {}",
            v["name"],
            v["phones"][0]
        );
    }
```
Macro usage
===========

Since Rust 2018 you no longer have to explicitly import macros, they
work just like functions.

Simply call a macro as follows
```rust,ignore
    use bar::baz;

    fn main() {
            baz!(); // I am a macro!
    }
```
You might still encounter older Rust code which relies on `#[macro_use]`

!
=

Crates can be renamed, just like normal use statements:
```rust,ignore
    use serde_json as json;

    fn main() {
            crate::json::some_function();
    }
```
Note the `crate` keyword before its usage. You’re using a local alias.
The original still exists.

!
=

Alternative: rename dependencies via cargo:
```toml
    [dependencies]
    foo = { git = "svnexpress.com/user/mylib", package = "foo" }
    bar = { version = "0.1", package = "actually_its_bar_these_days" }
```
crates.io
=========

Published crates can be found on [crates.io](https://crates.io), their
documentation is automatically published to [docs.rs](https://docs.rs).

Crates and Applications
=======================

`cargo install my_crate` installs the applications shipped with a crate.
