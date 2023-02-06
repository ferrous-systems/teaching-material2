[Table of Contents](./index.html)

!
=

Rusts iterators are:

-   Lazy

-   Pervasive

-   Potentially infinite

Where Do They Come From?
========================

-   Collections like `Vec<T>` have an `iter()` function which yields an
    iterator.

-   Things like `std::net::TcpListener` which provides an iterator of
    \`\`TcpStream\`\`s via \`\`incoming()\`\`.

-   Functions like `str::split` and `str::split_n`

-   Iterators can be implemented on other structures as well.

Owned iterators
===============

    fn main() {
        let vec = vec![1,2,3];
        let iter = vec.into_iter();

        for i in iter {
            println!("{}", i);
        }

        //println!("{:?}", vec); 
    }

-   this won’t work

Borrowed iterators
==================

-   point out that the `for` loop "unwraps" all \`Some()\`s returned by
    the iterator for us

<!-- -->

    fn main() {
        let vec = vec![1,2,3];
        let iter = vec.iter();

        for i in iter {
            println!("{}", i);
        }

        println!("{:?}", vec);
    }

Mutably Borrowed iterators
==========================

    fn main() {
        let mut vec = vec![1,2,3];
        let iter_mut = vec.iter_mut();

        for i in iter_mut {
           *i += 1
        }

        println!("{:?}", vec);
    }

Conventions
===========

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
<td><p>.into_iter()</p></td>
<td><p>.iter()</p></td>
<td><p>.iter_mut()</p></td>
</tr>
</tbody>
</table>

Common Uses
===========

`next()`
========

-   ask participants what they think this returns

-   point out that we see `` Some()`s and a `None `` here because
    there’s not for loop to unwrap the return value of `next()` for us

Iterators can be advanced manually:

    fn main() {
        let items = vec![0, 1, 2];
        let mut iterator = items.into_iter();
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
    }

Combinators: `map()`
====================

Transform items as they are evaluated:

    fn main() {
        let fizzbuzz = (0..10_000)
            .map(|x| match x {
                x if x % 15 == 0 => String::from("Fizzbuzz"),
                x if x % 3  == 0 => String::from("Fizz"),
                x if x % 5  == 0 => String::from("Buzz"),
                x => format!("{}", x),
            });
        for item in fizzbuzz {
            println!("{}", item);
        }
    }

Combinators: `filter()`
=======================

Filter out unwanted values, skipping further computation on them:

    fn main() {
        let evens = (0..10_000)
            .filter(|x| x % 2 == 0);
        for item in evens {
            println!("{}", item);
        }
    }
