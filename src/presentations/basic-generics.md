# Basic Generics

TODO: Move to "Applied Rust"
TODO: Fix naming inconsistency in folder/presentation/filename/kebab vs snake case?

Generics are fundamental for Rust.

Generic Structs
----

```rust,editable
    struct Point<Precision> {
        x: Precision,
        y: Precision
    }

    fn main() {
        let point = Point { x: 1_u32, y: 2 };
        let point: Point<i32> = Point { x: 1, y: 2 };
    }
```
Type Inference
----

Rust finds the types of all variables and generics with sufficient
information.

This only applies **inside** of function bodies.

Signatures must always be fully specified.

Generic Enums
----

-   under the hood: the compiler silently copies for all instances of
    `Either` → the compiler does the boilerplate-ing for you!

-   similar-ish to C++ templates

<!-- -->
```rust,editable
    enum Either<T, X> {
        Left(T),
        Right(X),
    }

    fn main() {
        let alternative: Either<i32, f64> = Either::Left(123);
    }

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    enum Option<T> {
        Some(T),
        None,
    }
```
Generic Functions
----

-   the pattern is always `name<type>` regardless of whether name
    is a function name, parameter name, etc

Generic Functions have type parameters.
```rust,ignore,editable
    fn accept_any_type<T>(arg: T) {
        // ...
    }

    fn accept_and_return_any_type<T, U>(arg: T) -> U {
        // ...
    }
```
Generic functions are used when computations can be expressed in an
abstract way.

-   possible detour: monomorphization, generic contraints (+ where
    clauses)
