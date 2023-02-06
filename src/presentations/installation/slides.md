[Table of Contents](./index.html)

Rustup
======

Rustup installs and manages Rust compiler toolchains

<https://rust-lang.org/tools/install>

**It is not the Rust compiler!**

Important commands
==================

    # Installation of a toolchain (here: the stable release channel)
    $ rustup install stable

    # Selection of a default toolchain
    $ rustup default stable

    # Display documentation in browser
    $ rustup doc [--std]

    # Override the default toolchain in your directory
    $ rustup override set stable

    # List supported targets
    $ rustup target list

    # Add and install a target to the toolchain (here: to cross-compile for an ARMv6-M target)
    $ rustup target add thumbv6m-none-eabi

For up-to-date information, please see [Rust Component
History](https://rust-lang.github.io/rustup-components-history/)

Contents of the toolchain
=========================

-   rustc, rust-docs, rust-std, rustfmt, rust-(lldb|gdb)<sup>\*</sup>

-   cargo, cargo-fmt

-   clippy

-   libcore/libstd

<sup>\*</sup> The debugger installed is platform dependent.

rustc
=====

    $ rustc --help

The Rust compiler builds and links Rust code.

The quintessential beginning
============================

    fn main() {
        println!("Hello, World!");
    }

The quintessential output
=========================

    $ rustc hello_world.rs
    $ ./hello_world
    Hello, World!

Cargo
=====

    $ cargo --help

Cargo contd.
============

Cargo is Rusts build and package management tool.

Cargo is installed along with `rustc`, but is not tightly coupled to a
`rustc` version.

Once more with Cargo
====================

    $ cargo new hello-world
    $ cd hello-world
    $ cat src/main.rs
    fn main() {
        println!("Hello, world!");
    }
    $ cargo build
       Compiling hello-world v0.1.0 (file:///Users/skade/Code/rust/scratchpad/hello-world)
        Finished debug [unoptimized + debuginfo] target(s) in 0.35 secs
    $ cargo run
        Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target/debug/hello-world`
    Hello, world!

A Little Look Around
====================

-   What is in Cargo.toml?

-   What is in Cargo.lock?

For details, check the [Cargo Manifest
docs](http://doc.crates.io/manifest.html).

Playground
==========

-   Playground: <https://play.rust-lang.org>

IDEs
====

-   rust-analyzer: <https://rust-analyzer.github.io>

-   IntelliJ Rust plugin for their IDEs (CLion, Idea, etc.):
    <https://www.jetbrains.com/rust/>

rust-analyzer: Things to know
=============================

-   With VSCode you need this extension -
    <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>

-   You must remove the default (now deprecated) Rust extension if you
    have that installed

-   rust-analyzer helps you have those IDE-like features

Add some tooling
================

    $ rustup component add rustfmt
    $ rustup component add clippy
    $ cargo tree
