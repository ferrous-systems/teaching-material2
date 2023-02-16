[Table of Contents](./index.html)

Declaration
===========

    fn add(first: i32, second: i32) -> i32 {
        first + second
    }

Arguments
=========

```rust
    fn return_nothing() {}

    fn return_nothing_as_well() -> () {
        println!("Hello!");
    }

    fn return_a_random() -> i32 {
        4 // Chosen by dice roll.
    }

    fn return_max(should: bool) -> Option<i32> {
        if x > y {
            x
        } else {
            y
        }
    }
```

Returning
=========

Returning is optional. Signatures must be complete. Note the `;`!
```rust
    fn doesnt_return() {
        true;
    }

    fn doesnt_return_either() -> () {
        true;
        ()
    }

    fn doesnt_compile() -> bool {
        true;
    }

    fn returns() -> bool {
        true
    }
```

TODO: Move rest of slides to "Applied Rust"
Generic Functions
=================

Generic functions have type parameters.
```rust
    fn takes_anything<T>(thing: T) -> T {
        thing
    }
```
With Bounds
===========

Generic functions can be *constrained*.

These are equivalent:

```rust
    fn prints_anything<T: Debug>(thing: T) {
        println!("{:?}", thing);
    }

    fn prints_anything<T>(thing: T)
    where
        T: Debug,
    {
        println!("{:?}", thing);
    }
```
Functions for types
===================

If we didn’t have methods (like in C), we’d have to write this:

```rust
    struct Square(f32);

    fn square_num_sides() -> u32 {
        4
    }

    fn square_calc_area(square: &Square) -> f32 {
        square.0 * square.0
    }

    fn square_double_size(square: &mut Square) {
        square.0 *= 2.0;
    }
```
Associated Functions
====================

Fortunately, Rust has a better solution than putting `square_` on all
our function names.

```rust
    struct Square(f32);

    impl Square {
        fn num_sides() -> u32 {
            4
        }
    }

    fn main() {
        println!("Num sides: {}", Square::num_sides());
    }
```
Methods that access data
========================

When our function needs to access the data inside the associated type,
we can use `&self`.

This is a shortcut for `self: &Self`, where `Self` is an alias for
whatever `impl Foo` block we’re inside of.

```rust
    struct Square(f32);

    impl Square {
        fn calc_area(&self) -> f32 {
            self.0 * self.0
        }
    }

    fn main() {
        let sq = Square(1.0);
        println!("Area: {}", sq.calc_area());
    }
```
Methods that mutate data
========================

When our function needs to mutate the data inside the associated type,
we can use `&mut self`.

This is a shortcut for `self: &mut Self`, where `Self` is an alias for
whatever `impl Foo` block we’re inside of.

```rust
    struct Square(f32);

    impl Square {
        fn calc_area(&self) -> f32 {
            self.0 * self.0
        }

        fn double_size(&mut self) {
            self.0 *= 2.0;
        }
    }

    fn main() {
        let mut sq = Square(1.0);
        println!("Area: {}", sq.calc_area());
        sq.double_size();
        println!("New Area: {}", sq.calc_area());
    }
```
Methods that taken ownership of the data
========================================

When our function needs to take ownership of the variable associated
type, we can use `self`.

This is a shortcut for `self: Self`, where `Self` is an alias for
whatever `impl Foo` block we’re inside of.

```rust
    struct Square(f32);

    impl Square {
        fn calc_area(&self) -> f32 {
            self.0 * self.0
        }

        fn destroy(self) {
            println!("I ate the square. It has gone.");
        }
    }

    fn main() {
        let sq = Square(1.0);
        println!("Area: {}", sq.calc_area());
        sq.destroy();
        // This line won't compile
        // println!("Area: {}", sq.calc_area());
    }
```
