# Macros

!
=

Rust has a macro language. See [The Little Book Of
Macros](https://danielkeep.github.io/tlborm/book/README.html).

Important Macros
----

-   `println!(pattern, [values])` Easy printing of formatted strings to
    stdout

-   `format!(pattern, [values])` like `println!`, but returns Strings

-   `write!(buf, pattern, [values])` Simple writing of formatted data to
    a buffer

What Can They do?
----

Macros can be used to things such as:

-   Generate repetitive code.

-   Create DSLs.

-   Write things that would otherwise be hard without Macros.

What do They Provide?
----

Macros are:

-   **Hygienic**, expansion happens in a different *syntax context*

-   **Correct**, they cannot expand to invalid code.

-   **Limited**, they cannot, for example, pollute their expansion site.

Components of a Macro
----

A macro has three parts.

-   A name, eg. `println`.

-   A input portion, defining what the macro accepts.

-   An output portion, defining how it expands.

Macros: Syntax
----

Let’s see how they look:
```rust
    macro_rules! double {
      // Input parameters
      ($value:expr)
      =>
      // Output
      ($value * 2);
    }

    fn main() {
        let doubled = double!(5);
        println!("{}", doubled);
        // Alternatives:
        double! { 5 };
        double![5];
    }
```
Macros: Syntax
----

The `($value:expr)` part says that:

The macro accepts one parameter which is an expression.

Parameter types can be restricted.

For example, `$foo:ty` only accepts a type.

Macros: Syntax
----

The parameters are prepended with a `$` to distinguish them.

Both in the input and output.

Macros: Repetitions
----
```rust
    macro_rules! implement_foo_for {
        [
            // This is a repetition!
            $($implement_for:ty,)*
        ] => {
            // This iterates over the repetition!
            $(impl Foo for $implement_for {})*
        }
    }

    implement_foo_for![u8, u16, u32, u64, usize,];
    implement_foo_for! { i8, i16, i32, i64, isize, }
    implement_foo_for!(f32, f64,);

    trait Foo {
        fn foo(&self) {
            println!("Foo!");
        }
    }

    fn main() {
        1_u8.foo();
        1_u16.foo();
    }
```
Macros: Repetitions
----

When we see `$(...)*` this is signalling a repetition. It communicates:

This portion of the macro takes a variable number of arguments.

Each repetition in the input should have a matching one in the output.

Macros: Custom Syntax
----
```rust
    macro_rules! email {
        ($user:expr => $domain:expr) => {
            format!("{}@{}", $user, $domain);
        }
    }

    fn main() {
        let address = email!("me" => "example.org");
        println!("{}", address);
    }
```
Macros: Custom Syntax
----

Macros allow for a limited form of custom syntax and can be used to
build simple DSLs.

For a good example of this, see
[clap-rs](https://github.com/kbknapp/clap-rs)

Downsides of Macros
----

Macros:

-   Can be difficult to debug.

-   Can be confusing to read and understand.

When Should You Use Macros?
----

Use macros where there are no other good alternatives.

Avoid overusing macros.
