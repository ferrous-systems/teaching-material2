[Table of Contents](./index.html)

!
=

To check variants of enums, `match` is used.

!
=

    fn main() {
        let mut args = std::env::args();

        match args.nth(1) {
            Some(arg) => { println!("Argument: {}", arg)},
            None => { println!("No Argument") }
        }
    }

Alternative: if-let
===================

    fn main() {
        let mut args = std::env::args();

        if let Some(arg) = args.nth(1) {
           println!("Argument: {}", arg);
        }
    }

!
=

    fn main() {
        let maybe_file = std::fs::File::open("Not there!");

        match maybe_file {
            Ok(f) => { println!("File opened! Debug: {:?}", f) },
            Err(e) => { println!("File not opened!! Error: {:?}", e) }
        }
    }

!
=

Matches must cover all variants!

Ignoring variants
=================

    fn main() {
        let maybe_file = std::fs::File::open("Not there!");

        match maybe_file {
            Err(e) => { println!("Error: {:?}", e) }
            _ => {}
        }
    }

!
=

Results carry a special marker: they must not be ignored!

    fn main() {
        std::fs::File::open("Not there!");
    }

Solution: match or pass on.

!
=

`match` does not only work on enums:

    fn main() {
        let number = 4;

        match number {
            0 => println!("Number is 0"),
            _ => println!("Number is something else")
        }
    }

!
=

`match` and `if` expressions:

    fn main() {
        let mut args = std::env::args();

        let value = if let Some(arg) = args.nth(1) {
                        arg
                    } else {
                        "default!".to_string()
                    };
    }

!
=

`match` can be used on multiple expressions!

    fn main() {
        let n = 2400;
        match (n % 400, n % 100, n % 4) {
            (0, _, _) => true,
            (_, 0, _) => false,
            (_, _, 0) => true,
            _ => false,
        }
    }

**Remember**: `match` arms are evaluated sequentially - first correct
choice is chosen.
