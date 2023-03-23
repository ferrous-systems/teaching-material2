In this exercise you will learn

-   How to serialize and deserialize data with Serde crate.

-   When to use types that own their data or only refer to externally
    owned data

-   How to use `std::borrow::Cow` type to minimize extra memory
    allocations

Getting Started
----

Create a new binary project for this exercise. Add the following
dependencies to your `Cargo.toml` file:

    [dependencies]
    serde = { version = "1", features = ["derive"] }
    serde_json = "1"

Use this template:

    // pretend that we call an API and get a JSON String back
    fn fetch_data() -> String {
        String::from(
            r#"
                {
                    "id": 1,
                    "title": "Hello, Rust"
                }
            "#,
        )
    }

    #[derive(Debug)]
    struct BlogPost {
        id: u32,

        title: String,
    }

    fn main() -> Result<(), serde_json::Error> {
        let post: BlogPost = {
            let data = fetch_data();
            todo!("use `serde_json` crate to parse JSON")
        };
        println!("deserialized = {:?}", post);

        let post_json: String =
            todo!("use `serde_json` to convert `post` to a string");
        println!("serialized = {:?}", post_json);

        Ok(())
    }

`r` in front of a string literal means it’s a "raw" string. Escape
sequences (`\n`, `\"`, etc.) don’t work, and thus they are very
convenient for things like regular expressions, JSON literals, etc.

Optionally `r` can be followed by one or more symbols (like `\#` in our
case), and then your string ends when there’s a closing double quote
followed by the same number of the same symbols. This is great for cases
when you want to have double quotes inside your string literal. For our
example `r#" ... "#` works great for JSON. In rare cases you’d want to
put two or more pound signs. Like, when you store CSS color values in
your JSON strings:

    // here `"#` would not terminate the string
    r##"
        {
            "color": "#ff00ff"
        }
    "##

Tasks
----

1. Serialization and Deserialization using `String` s
-----------------------------------------------------

We used `todo!()` macros to mark places where you should put code to
make the program run. Look at
[`serde_json`](https://docs.rs/serde_json/latest/serde_json/#functions)
api for help.

Serde comes with two traits: `Serializable` and `Deserializable`. These
traits can be `derive` d for your `struct` or `enum` types. Other
`serde-*` crates use these traits to convert our data type from and to
corresponding representation (`serde-json` to JSON, `serde-yaml` to
YAML, etc.).

2. Deserialization using `str` s
--------------------------------

Notice that when we deserialize data we created a separate `String` to
represent a `name` for our blog post. If our data type has a lot of
fields in it we may end up allocating many-many Strings every time we
receive data from our API.

To avoid it we often can use string slices to refer to fragments of the
original JSON string.

Try changing or blog post type to the following:

    struct BlogPost {
        id: u32,

        title: &str,
    }

The compiler will ask you to provide a lifetime type annotation.

**Q: Can your `title` be `&'static str`? Why or why not?**

Use lifetime annotations and change the code inside `main` if necessary.

**Q: If you made a change, why?**

3. Use copy-on-write.
---------------------

Let’s put the word *Rust* into quotes in our blog post title. Double
quotes have to be escaped in JSON, so try changing the title to
`"Hello, \"Rust\""`.

Run your program, and you’ll get an error:

    Error("invalid type: string \"Hello, \\\"Rust\\\"\", expected a borrowed string", ... )

[`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
can be used to represent data that can be borrowed (referenced) or
owned. It provides Copy-on-Write semantics.

Change `title` type to `Cow<'a, str>`. The error should disappear.

4. Teach Serde to not do unnecessary allocations.
-------------------------------------------------

Let’s observe `Cow` behavior. Add this to your `main` function after
deserialization step:

    println!("is borrowed = {}", matches!(post.title, Cow::Borrowed(_)));

Now change the title of the blog post back to `"Hello, Rust"` - without
double quotes, and run the program again.

Turns out Serde creates an owned copy of the data anyway. To avoid that
use [`#[serde(borrow)]`](https://serde.rs/field-attrs.html#borrow)
macro.

Get to the point when your program prints: `is borrowed = true`

Questions after this exercise
----

1.  Why is there a need for separate `serde` and `serde-json` crates?

2.  Why do we need to put a lifetime annotation for a field of *&str*
    type?

3.  Why do we need to put a lifetime annotation for a field of *Cow*
    type?

4.  When can we use plain `&str` vs `Cow<str>` or `String` for text data
    during deserialization?

5.  What are pros and cons of using `&str` or `Cow<str>` compared to
    `String?`
