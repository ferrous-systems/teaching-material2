[Table of Contents](./index.html)

Imports
=======

All used items must be declared. This is similar to Java or Haskell.

    use std::fs::File;

    fn main() {
        let f = File::open("test");
    }

Module Imports
==============

It is possible to import the module instead and qualify every use.

    use std::fs;

    fn main() {
        let f = fs::File::open("test");
    }

Glob Imports
============

You can also import everything from a module.

    use std::fs::*;

    fn main() {
        let f = File::open("test");
    }

This is **generally** frowned upon.

Prelude
=======

One exception to the rule is the "Prelude": This is a special module in
the standard library that is automatically fully imported.

Other Preludes
==============

Other libraries offer `prelude`-Modules, one of the most common is
`std::io`.

    use std::io::prelude::*;
    use std::fs::File;

    fn main() {
        let mut f = File::open("foo.txt").unwrap();
        let mut buffer = [0; 10];

        f.read(&mut buffer).unwrap();

        println!("The bytes: {:?}", buffer);
    }

Here, the glob is accepted.

Structured imports
==================

You can combine multiple things, that are also nested.

    use std::{fs::File, io::{Read, Write}};

    fn main() {
        let mut buffer = [0; 10];

        let mut f1 = File::open("foo.txt").unwrap();
        f1.read(&mut buffer).unwrap();

        let mut f2 = File::create("bar.txt").unwrap();
        f2.write_all(&buffer).unwrap();
    }

Renaming on import
==================

    use std::fs as file_system;

    fn main() {
        let f = file_system::File::open("test");
    }

Local import
============

Imports can happen inside a function. They only take effect within the
function.

    fn main() {
        use std::fs::File;

        let f = File::open("test");
    }

Modules
=======

The module system of Rust is similar to Python.

!
=

-   every source file is a module

-   submodules can be in the same file or in another

-   libraries are called "crates" and contain modules

!
=

By convention, the root module of a library is found in `src/lib.rs`.

the root module of a single application in `src/main.rs`.

The root modules for multiple applications in `src/bin/*.rs`.

Example
=======

    fn main() {
        workload::work();
    }

    mod workload {
        pub fn work() {
            println!("hard at work!");
        }
    }

Moving the Module to a Separate File
====================================

Our application could also have the following layout:

    |
    |-src
      |- main.rs
      |- workload.rs

A Larger Module as a Directory
==============================

Simply by adding a new folder of the same name

    |
    |-src
      |- main.rs
      |- workload.rs
      |- workload/
         |- thing.rs

A Larger Module as a Directory
==============================

Or declare a module via `mod.rs`

    |
    |-src
      |- main.rs
      |- workload
         |- mod.rs
         |- thing.rs

!
=

In both cases, the module must be registered with the root module.

    mod workload;

    fn main() {
        workload::work();

        workload::thing::do_stuff();
    }

!
=

With the last approach, you can create additional modules relative to
`mod.rs`.

Visibility
==========

In Rust, everything is private by default. Publicly available types are
marked with `pub`.

Public types and functions that can be reached through a public module
path are exported.

Example
=======

    pub mod workload;

    pub trait Distance {
        fn distance(&self, other: Self);
    }

    pub fn foo() {

    }

!
=

Traits must be public and imported before use.

The compiler will detect if you use an un-imported trait.

Structs
=======

Struct fields are not exported (public) by default. A struct with
non-public members can’t be constructed or fully used outside of its
module. This is often intended.

Struct functions are also not exported by default.

!
=

    pub struct Point {
        x: i32,
        y: i32
    }

    impl Point {
        pub fn new() -> Point {
           Point { x: 1, y: 2 }
        }
    }

!
=

    pub struct Point {
        pub x: i32,
        pub y: i32
    }

!
=

In general, exporting fields should be avoided:

-   Any change of the structure leads to API breakage

-   Accessor functions are usually as fast as direct field access due to
    optimizations.

Pub qualifiers
==============

    pub(crate) fn crate_local() {

    }

    pub(super) fn visible_in_super_module() {

    }
