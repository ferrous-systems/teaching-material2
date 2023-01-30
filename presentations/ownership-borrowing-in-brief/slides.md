-   stress that this is central to understanding Rust and that we will
    spend as much time on it as needed until everyone feel like they
    have a grasp on it

[Table of Contents](./index.html)

!
=

Ownership is the basis for the memory management of Rust.

Rules
=====

-   you can use the metaphor of a physical book: I own it, I can decide
    to mutate it (e.g. by coloring in it or not), I’m responsible for
    its whereabouts and disposal

<!-- -->

-   Every value has exactly one owner

-   Ownership can be passed on, both to functions and other types

-   The owner is responsible for removing the data from memory

-   The owner has all powers over the data and can mutate it

!
=

These rules:

-   are fundamental to Rust’s type system

-   are enforced at compile time

-   are practical in many other ways

Example
=======

    use std::fs::File;
    use std::io::Write;
    use std::io;

    fn main() -> io::Result<()> {
        let file_create = File::create("test"); // 

        let mut file = match file_create {  // 
            Ok(f) => f, // 
            Err(e) => panic!("File create failed: {}", e),
        };

        file.write_all(b"Hello World!")
    }  // 

-   Tries to open a file

-   Checks the Result of the opening operation

-   Take ownership of the file handle

-   Remove the handle and close it in the process

Ownership passing
=================

    use std::fs::File;
    use std::io::Write;
    use std::io;

    fn main() -> io::Result<()> {
        let file_create = File::create("test");

        let file = match file_create {
            Ok(f) => f,
            Err(e) => panic!("File create failed: {}", e),
        };

        write_and_close(file) // 
    }

    fn write_and_close(mut file: File) -> io::Result<()> { // 
        file.write_all(b"Hello World!")

        // 
    }

-   Ownership is passed here.

-   The value dropped here.

First safety checkpoint
=======================

    use std::fs::File;
    use std::io::Write;
    use std::io;

    fn main() -> io::Result<()> {
        let file_create = File::create("test");

        let file = match file_create {
            Ok(f) => f,
            Err(e) => panic!("File create failed: {}", e),
        };

        write_and_close(file);
        write_and_close(file) // 
    }

    fn write_and_close(mut file: File) -> io::Result<()> {
        file.write_all(b"Hello World!")
    }

-   This is illegal.

Oops!
=====

    8  |     let file = match file_create {
       |         ---- move occurs because `file` has type `std::fs::File`, which does not implement the `Copy` trait
    ...
    13 |     write_and_close(file);
       |                     ---- value moved here
    14 |     write_and_close(file)
       |                     ^^^^ value used here after move

!
=

In Rust-Lingo, this is called `consuming`.

The value cannot be used anymore.

Background
==========

When calling `write_and_close` with `file`, the value is "moved" into
the arguments of `write_and_close`.

At that moment, ownership passes to `write_and_close`.

`main` is not owner of the data anymore and thus not allowed to access
or manipulate them.

References & Borrowing
======================

-   book metaphor: I can mutably borrow my book to a friend to color in
    it read it, and then immutably borrow it to another to look at the
    pretty colors

Intuitively: what you own, you can borrow.

---

    use std::fs::File;
    use std::io::Write;
    use std::io;

    fn main() -> io::Result<()> {
        let file_create = File::create("test");

        let mut file = match file_create {
            Ok(f) => f,
            Err(e) => panic!("File create failed: {}", e),
        };

        print_filelen(&file)?;
        file.write_all(b"Hello World!")?;
        print_filelen(&file)
    }

    fn print_filelen(f: &File) -> io::Result<()> {
        println!("len: {:?}", f.metadata()?.len());
        Ok(())
    }

Immutable references
====================

`&` is the so-called "immutable" reference. They are:

-   Available multiple times

-   Always valid (always pointing to living data)

-   Never `null`

-   Guaranteed to never observe mutation of the pointee

Mutable Borrowing
=================

    use std::fs::File;
    use std::io::Write;
    use std::io;

    fn main() -> io::Result<()> {
        let file_create = File::create("test");

        let mut file = match file_create {
            Ok(f) => f,
            Err(e) => panic!("File create failed: {}", e),
        };

        print_filelen(&file)?;
        write_to_file(&mut file);
        print_filelen(&file)
    }

    fn print_filelen(f: &File) -> io::Result<()> {
        println!("len: {:?}", f.metadata()?.len());
        Ok(())
    }

    fn write_to_file(f: &mut File) -> io::Result<()> {
        f.write_all(b"Hello World!")
    }

Mutable references
==================

-   hot take: **shared** mutable data is the root of all evil. mutable
    data itself is ok, as long as you follow the ownership rules!

`&mut` is the so-called "mutable" reference. They are:

-   Available only once at a time

-   Always valid (always pointing to living data)

-   Never `null`

-   Guaranteed to never alias (no two references point to the same data)

Values can be:

-   Borrowed immutably as often as you’d like

-   Or mutably exactly once at a time

-   The two rules are mutually exclusive.

Rust forbids *shared mutability*.

Types and their ownership behaviour
===================================

<table>
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<tbody>
<tr class="odd">
<td><p>Owned</p></td>
<td><p>Borrowed</p></td>
<td><p>Mutably borrowed</p></td>
</tr>
<tr class="even">
<td><p>i32</p></td>
<td><p>&amp;i32</p></td>
<td><p>&amp;mut i32</p></td>
</tr>
<tr class="odd">
<td><p>Point</p></td>
<td><p>&amp;Point</p></td>
<td><p>&amp;mut Point</p></td>
</tr>
<tr class="even">
<td><p>Box&lt;i32&gt;</p></td>
<td><p>&amp;i32</p></td>
<td><p>&amp;mut i32</p></td>
</tr>
</tbody>
</table>

Collections and their ownership behaviour
=========================================

<table>
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<tbody>
<tr class="odd">
<td><p>Owned</p></td>
<td><p>Borrowed</p></td>
<td><p>Mutably borrowed</p></td>
</tr>
<tr class="even">
<td><p>Vec&lt;i32&gt;</p></td>
<td><p>&amp;[i32]</p></td>
<td><p>&amp;mut [i32] or &amp;mut Vec&lt;i32&gt;</p></td>
</tr>
<tr class="odd">
<td><p>String</p></td>
<td><p>&amp;str</p></td>
<td><p>&amp;mut str or &amp;mut String</p></td>
</tr>
</tbody>
</table>

Working with moves: explicit clone
==================================

What if ownership behaviour is getting messy, but we don’t want to
reference?

We can create a second copy of the data!

!
=

    #[derive(Debug, Clone)]
    struct Dot {
        x: i32,
        y: i32
    }

    fn main() {
        let dot = Dot { x: 1, y: 2 };
        pacman(dot.clone());
        pacman(dot);
    }

    fn pacman(dot: Dot) {
        println!("Eating {:?}", dot);
    }

!
=

Cloning is a general operation that - depending on the complexity of the
data at hand - can be costly.

Working with moves: copy instead of move
========================================

    #[derive(Debug, Clone, Copy)]
    struct Dot {
        x: i32,
        y: i32
    }

    fn main() {
        let dot = Dot { x: 1, y: 2 };
        pacman(dot);
        pacman(dot);
    }

    fn pacman(dot: Dot) {
        println!("Eating {:?}", dot);
    }

!
=

Copy is meant for data that can be quickly copied in memory (using
memcopy) and are allowed to be copied (e.g.: not File pointers).

!
=

Values that are copy follow the standard ownership rules, but they are
copied when ownership is passed on.

Warning
=======

The terminology around moves is similar, but not the same to the one
used in C++, which is why you should always use Rust-Terminology:
Ownership, passing on ownership and consumption.

Small quiz
==========

`drop` is the function that deallocates a value immediately. What does
the implementation look like?

    use std::fs::File;

    fn main() {
        let file = File::open("test").unwrap();
        let buffer = read_from(&file);
        drop(file);
        // do something long
    }

!
=

    #[inline]
    fn drop<T>(_: T) {
      // take ownership, drop out of scope
    }
