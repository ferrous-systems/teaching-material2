[Table of Contents](./index.html)
```rust
use std::io; // 
use std::io::prelude::*;
use std::fs::File;

fn main() -> Result<(), io::Error> { // 
    let open_file = File::open("test"); // 

    let mut file = match open_file { // 
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut buffer = String::new(); // 
    file.read_to_string(&mut buffer)?; // 
    println!("{}", buffer);

    Ok(()) // 
}
```

A Little Bit of History
=======================

-   Rust began around 2008

-   An experimental project by Graydon Hoare

-   Adopted by Mozilla

-   Presented to the general public as version 0.4 in 2012

-   Looked a bit Go-like back then

Focus
=====

-   Rust has lost many features from 2012 to 2014

-   Garbage collector, evented runtime, complex error handling

    -   All present once, now gone

-   Orientation towards a usable systems programming language

Development
===========

-   Always together with a larger project (e.g. Servo)

-   Early adoption of regular releases, deprecations and an RFC process

Release Method
==============

-   Nightly releases

    -   experimental features are only present on nighly releases

-   Every 6 weeks, the current nightly is promoted to beta

-   After 6 weeks of testing, beta becomes stable

-   Guaranteed backwards-compatibility

-   Makes small iterations easier

Goals
=====

-   Explicit over implicit

-   Predictable runtime behaviour

-   Supporting stable software development for programming at large

-   Pragmatism and easy integration

-   Approachable project

Rust wants to be usable first!
==============================

!
=

Many examples in this course are very small, which is why we will also
spend time discussing the impact of many features on large projects.

The Three Words
===============

-   Safe

-   Concurrent

-   Fast

Safe
====

-   Rust is memory-safe

-   No illegal memory access

-   Deallocation is automated

-   Warning: memory leaks are **safe** by that definition!

Concurrent
==========

-   "Concurrency without fear"

-   The type system detects concurrent access to data and requires
    synchronisation

-   Also: Rust detects when unsynchronised access is safely possible!

-   Protection from data races!

Fast
====

-   These properties are guaranteed at compile time and have no runtime
    cost!

-   Optimizing compiler based on LLVM

-   Features with runtime cost are explicit and hard to activate "by
    accident"

-   No reflection

-   Zero-cost abstractions

-   "Pay what you use": Rust has features with a runtime cost in an
    explicit and visible way. Unused features do not come with an
    associated cost.

Pragmatic
=========

-   Great tooling

-   Sublanguage for unsafe memory access and techniques to handle these

-   Great FFI support

-   Great error messages by the compiler

-   Rust gives support for many hard things and trusts you with the rest

Where do Rustaceans come from?
==============================

From diverse backgrounds:

-   Dynamic languages (JS, Rubyists and Pythonistas)

-   Functional languages like Haskell and Scala

-   C/C++
