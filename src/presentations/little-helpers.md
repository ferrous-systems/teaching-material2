# Little Helpers

TODO: Move to "Applied Rust"?

Some collected hints to get you started.

```rust, ignore,editable
    #[derive(Eq, PartialEq, Debug)] 
    pub struct Point { 
        x: i32,
        y: i32,
    }
```

-   Derives allow to generate some standard functionality

-   Any type can carry a visibility modifier to export them

<!-- -->
```rust,editable
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 1, y: 2 };
        println!("{:?}", p); 
        println!("{:#?}", p); 
    }
```

-   Debug makes the `Debug` formatting string work

-   There’s also a more structured version, also enabled through it

<!-- -->
```rust,editable
    #[derive(Eq,PartialEq,Debug)]  
    struct Point {
        x: i32, 
        y: i32,
    }

    fn main() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 1, y: 2 };
        if p1 == p2 { 
            println!("The same!");
        }
        assert_eq!(p1, p2); 
    }
```
-   `Eq` describes total equality: for every pair of values, equality is
    defined

-   `PartialEq` is enough for getting `==`

-   Both can only be derived if all inner fields are both

-   Equality in action!

-   The `assert_eq!` compares to values and panics if they don’t match!

Unwrap Results and Option
----

If you expect something to work or an item to be there, use `unwrap`:

```rust,does_not_compile,ignore,editable
    use std::fs::File;

    fn main() {
        let file: File = File::open("Cargo.toml").unwrap();
    }
```

This expects the operation to have worked. You can add structured error
handling later.

Strings and their slices
----

Strings and string slices work much the same.
```rust,editable
    fn main() {
        let slice: &str = "Hello world!";
        let string: String = String::from(slice);
    }
```
In the beginning, habitually use `String`.
```rust,ignore,editable
    struct Owned {
        string_data: String
    }

    fn returns_string() -> String {
        String::from("Hello World")
    }

    fn hello(who: &str) -> String {
        format!("Hello, {}!", who)
    }

    use my_library::my_function; 

    #[test]
    fn my_test() {
        assert_eq!(1, 1);
    }

    #[test]
    #[should_fail]
    fn failing_test() {
        assert_eq!(1, 2);
    }
```

Rust and Cargo allows you to easily provide test for your code.

These can be put either directly in the source file or in any file in
`tests`.

-   Only needed when putting files in `tests`.

<!-- -->
```rust,editable
    fn addition(a: i32, b: i32) -> i32 {
        todo!()
    }

    #[test]
    fn addition_test() {
        assert_eq!(addition(1,2), 3);
    }
```
