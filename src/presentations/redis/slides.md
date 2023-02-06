[Table of Contents](./index.html)

!
=

Let’s revisit & deepen our knowledge of a few areas.

Cargo
=====

A Cargo package can (at the same time) contain:

-   0 or 1 library crates

-   0..n binary crates

it must contain at least one crate in total.

Cargo binaries
==============

-   The default binary is the `main()` function in `src/main.rs`.

-   Additional binaries are stored in the `src/bin/` directory.

-   Example binaries are located in `examples/`.

Cargo libraries
===============

By default, the library crate is implemented in `src/lib.rs` and has the
same module name as the package.

Cargo helpers
=============

Cargo can be extended:

-   `cargo-tree`: show dependency graph

-   `cargo-edit`: add/remove/upgrade dependencies

-   `cargo-watch`: recompile/restart project on change

and many more.

Cargo crates
============

<https://crates.io> is the public crates registry. Dependencies
specified by version only in `Cargo.toml` will be downloaded and
installed from there.

Cargo crates (cont.)
====================

Cargo also allows you to specify `git` or `path` (local) dependencies:

-   `rand = { git = "https://github.com/rust-lang-nursery/rand", branch = "next" }`

-   `hello_utils = { path = "hello_utils" }`

Error handling: anyhow
======================

[anyhow](https://docs.rs/anyhow/1.0.40/anyhow/index.html) is a popular
crate for idiomatic error handling, simplifying common cases to quickly
get a running. Its counterpart by the same author is
[thiserror](https://docs.rs/thiserror/1.0.24/thiserror/) which aids in
fleshing out custom error types.

Ref: [some discussion on the philosophy of error handling & reporting in
Rust](https://nick.groenen.me/posts/rust-error-handling/)

Handling command line arguments
===============================

-   Rudimentary handling can be done using `std::env::args`

-   A feature-rich alternative is
    [structopt](https://docs.rs/structopt/0.3.21/structopt/).
