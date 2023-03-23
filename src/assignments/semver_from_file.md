In this exercise, you will learn how to

-   open a file

-   handle errors using the `Result`-type

-   use the `Option`-type

-   read a file line by line

-   use standard library features to quickly parse lines to structs.

Both the `Option` and `Result` are similar in a way. Both have two
variants, and depending on what those variants are, the program may
continue in a different way.

The Option Type can have the variant `Some(<some other type>)` or
`None`. It is used, when you have to handle optional values, for example
if you want to be able to leave a field of a struct empty, go assign the
option type to it. If the field has a value, it is `Some(<value>)`, if
it is empty, it is `None`.

The variants of the Result type are `Ok()` and `Err(e)`. It is used to
handle errors. If an operation was successful, `Ok()` is returned.
`Ok()` can be empty or have a return value. `Err(e)` contains an error
message that can be printed.

Both types can be used with the `match` keyword. The received value is
matched on patterns, each leads to the execution of a different
expression.

    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }

Template
----

Clone the teaching material repository at
[github.com/ferrous-systems/teaching-material](https://github.com/ferrous-systems/teaching-material).

A template with example data can be found in folder
[assignments/semver/parse\_file/template](https://github.com/ferrous-systems/teaching-material/tree/main/assignments/semver/parse_file/template).
It already imports (`use` s) all required items.

Task: see template for details
----
