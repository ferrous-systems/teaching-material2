# Closures

!
=

Rust has closures. Multiple, even.

-   **Advantage:** Highly optimized using only the absolutely necessary
    runtime resources, often none.

-   **Disadvantage:** Knowing the specific closure type is not always
    easy.

Notation
----

```rust
    fn main() {
        let vec = vec![1,2,3];
        let out = vec.iter().map(|x| x * 2).collect::<Vec<_>>();
        println!("{:?}", out);
    }
```

```rust
    fn main() {
        let vec = vec![1,2,3];
        let double = |x| { x * 2 };
        let out = vec.iter().map(double).collect::<Vec<_>>();
        println!("{:?}", out);
    }
```
Closure Types
----

Moving  
The closure consumes its environment. It can only be called once.

Mutating  
The closure mutates its environment

Referencing  
The closure references its environment immutably

!
=

`rustc` infers the type automatically, but it is needed for type
signatures!

Notation of closure arguments
----

```rust
    fn call_with_one<F>(some_closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        some_closure(1)
    }
```
Moves and Closures
----

To resolve ambiguity, closures borrow by default. Moving needs to be
requested.

```rust
    fn main() {
        let num = 5;

        let owns_num = move |x: i32| x + num;
    }
```
