# Mutability

!
=

Modern programming languages differ in their attitude towards data
mutability.

Where does Rust land?

An Example
----

```rust,does_not_compile, ignore, editable
    fn main() {
        let answer = 42;
        answer = 32;
    }
```
Correct
----
```rust,editable
    fn main() {
        let mut answer = 42;
        answer = 32;
    }
```
!
=

In Rust data mutability must be declared.

Mutability is always apparent from reading the code.

!
=

Mutability is fundamental to Rust and is a common consideration.

!
=

Mutability is a property of variables and bindings, not of the bound
data!
