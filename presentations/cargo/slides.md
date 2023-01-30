[Table of Contents](./index.html)

!
=

Cargo is the standard build toolchain for Rust.

Every release of `rustc` is shipped with a specific version of cargo.

In general, cargo is independent of the used `rustc` version.

Projects
========

A cargo project contains (at minimum):

-   A Manifest (Cargo.toml)

-   A source file (library or application) in the `src` folder (default)

!
=

In addition, a cargo project can contain:

-   tests (default directory `tests`)

-   benchmarks (default directory `benches`)

-   a build script (default `build.rs`)

-   examples (default directory `examples`)

The Manifest
============

The [Cargo Manifest](http://doc.crates.io/manifest.html) is written
using [TOML](http://doc.crates.io/manifest.html).

It at least contains the name of the project.

    [package]
    name = "tcp-mailbox"
    version = "0.1.0"
    authors = ["Florian Gilcher <florian.gilcher@ferrous-systems.com>"]

    [dependencies]
    async-std = "1" # would also choose 1.5
    clap = "2.2" # would also choose 2.3

`cargo build`
=============

`cargo build` builds the whole project and puts the result in the
directory `target`.

By default, cargo uses the Debug profile, which means that the resulting
binary with be unoptimized and with debug symbols!

`cargo build --release` uses the optimizing profile.

!
=

**Always use** `--release` for benchmarks, especially iterators get a
huge speed boost from optimizations.

`cargo run`
===========

If the project contains an application, you can run it using
`+cargo run -- [Arguments]+`.

If it contains multiple binaries, the name of the intended binary can be
given using `+--bin <name>+`.

The debug profile is default here.

`cargo test`
============

`cargo test` runs all tests.

You can provide a test name or module name to filter the test being run.

Tests use the debug profile per default.

`cargo test --workspace --examples` runs all tests, including
documentation test and compilation of examples!

`cargo bench`
=============

`cargo bench` runs all benchmarks.

Benchmarks default to the release profile.

Benchmarks can be written using the `criterion` crate.

Versioning
==========

In the Rust community [Semantic Versioning](https://semver.org) is the
recommended versioning standard.

-   The major version must be raised if there are breaking interface
    changes.

-   The minor version must be raised if new features are added

-   The patch version marks bug fixes and performance improvements.

Pre-release
===========

If the version of a library is below `1`, it is considered
"pre-release", which means breaking interface changes can happen with
**every minor version**, and in the `0.0.x` case, with every **patch**
version.

Version expressions
===================

Cargo allows expressing version ranges in different fashions.

-   `=1.2.3`: Exact version, cargo will only use that one

-   `0.1`: Any patch version of the "0.1" series

-   `1.0`: Any minor version of the "1.x" series

-   `< 0.8`: Any minor version smaller than "0.8"

For more details, check the [docs](http://doc.crates.io/manifest.html)

Version resolution
==================

At the first build (or through `cargo update`), cargo calculates a
version tree fulfilling the constraints stated in Cargo.toml.

If successful, it will be saved in a Lockfile (Cargo.lock).

The Lockfile should always be kept under version control for
applications.

For libraries, it can be checked in to provide a repeatable test
environment.

Dependencies in detail
======================

Cargo knows 3 different kinds of dependencies:

-   normal dependencies

-   build dependencies

-   development/test dependencies

Example
=======

    [dependencies]
    async-std = "1"

    [build-dependencies]
    cbindgen = "0.5"

    [dev-dependencies]
    quickcheck = "0.9"

Dependencies in detail
======================

    [dependencies.async-std]
    version = "1"
    git = "https://github.com/skade/async-std.git"
    branch = "my-new-feature"

    [dependencies.async-std]
    version = "1"
    path = "/my/local/path"

Aside
=====

Dependencies outside of crates.io are forbidden if a library is to be
published on crates.io.

Local paths
===========

It is possible to temporarily replace libraries with local ones.

For this, their path needs to be registered in
`$PROJECT_PATH/.cargo/config`.

    paths = ["/my/local/path", "/another/path"]

Libraries found here will be preferred. This allows easy testing of
patches.

Profiles
========

The cargo profiles (release, bench, test…) can be customized.

Details can be found in the
[Manifest-documentation](http://doc.crates.io/manifest.html).

Targets
=======

-   profiles and dependencies can be configured based on the compilation
    target

-   `+cargo build --target ...+` uses that target

-   the target must be installed beforehand

-   consider using xargo

Workspaces
==========

Cargo can group multiple projects in a workspace.

They then share settings and the same `target` directory.

See the [manifest documentation](http://doc.crates.io/manifest.html) for
details.

Features
========

`rustc` provides the ability to ignore certain code parts on
compilation.

This happens through feature flags.

    #[cfg(experimental)]
    fn amazing_function() {

    }

!
=

These can be registered in `Cargo.toml`.

    [features]
    default = []
    # Turns on experimental features.
    experimental = []

!
=

And then be expressed on a dependency:

    [dependencies.my_lib]
    version = "0.1"
    features = ["experimental"]

or used with `+cargo build --features experimental,other_feature`

Directly invoking `rustc`
=========================

`cargo rustc` invokes `rustc` directly and allows passing of additional
parameters.
