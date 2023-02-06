[Table of Contents](./index.html)

Cargo build scripts
===================

Some projects require additional steps to run before they can be built,
e.g. for code generation. Cargo supports this by means of a `build.rs`
file.

Cargo build scripts (cont.)
===========================

-   `build.rs` is itself written in Rust. It resides in the root
    directory of your package, **not** in `src/`.

-   it can have its own list of dependencies via `[build-dependencies]`

-   input is communicated via environment variables

-   output files should be written to `OUT_DIR`

-   Cargo behavior can be controlled by writing commands to stdout.

Cargo build scripts (cont.)
===========================

Generated code is typically then included in another Rust source file:

    pub mod generated {
        include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    }

Protocol buffers in Rust
========================

Several crates exist - we’re going to use
[prost](https://github.com/danburkert/prost) here.

Prost usage
===========

By using `prost::Message` you get encode & decode functionality.

    use prost::Message;
    // ...
    let mut buf = Vec::new(3);
    a_proto_object.encode(&mut buf)?;

    let another_proto_object = TheProtoObject::decode(a_vec.as_slice())?;
