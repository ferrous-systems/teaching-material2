In this exercise, we will implement a simple calculator.

You will learn:

-   How to write a simple Rust library

-   How to interact with borrowed and owned memory, especially how to
    take ownership

-   How to handle cases using the `match` syntax

-   How to create a safe parser in Rust manually

The library does not handle I/O. It’ll serve us as a pocket-sized
"`big`" project.

Syntax Specification
----

To simplify parsing, we will use postfix notation:

    expr = number
         | expr expr '+'
         | expr expr '-'
         | expr expr '*'
         | expr expr '/'
         | expr 'sqr'      # Squares expression

Here are some examples:

<table>
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<tbody>
<tr class="odd">
<td><p>Postfix notation</p></td>
<td><p>Infix notation</p></td>
<td><p>Value</p></td>
</tr>
<tr class="even">
<td><p><code>92</code></p></td>
<td><p><code>92</code></p></td>
<td><p>92</p></td>
</tr>
<tr class="odd">
<td><p><code>40 2 +</code></p></td>
<td><p><code>40 + 2</code></p></td>
<td><p>42</p></td>
</tr>
<tr class="even">
<td><p><code>1 3 + 2 /</code></p></td>
<td><p><code>(1 + 3) / 2</code></p></td>
<td><p>2</p></td>
</tr>
<tr class="odd">
<td><p><code>3 sqr 4 sqr + 5 sqr -</code></p></td>
<td><p><code>3^2 + 4^2 - 5^2</code></p></td>
<td><p>0</p></td>
</tr>
</tbody>
</table>

Postfix notation can be parsed in a straight-forward way with the help
of the stack data structure.

Basic Scaffold
----

Step 1  
Create a library project called `calc`: `cargo new --lib calc`

Step 2  
Define the Abstract Syntax Tree (AST) structure for postfix expressions.
Following the grammar, it needs to be able to represent:

-   numbers

-   one kind of unary expression (`sqr`)

-   four kinds of binary expressions

The best representation is a recursive `enum`:

    #[derive(Debug)]
    enum Expr {
        ...
    }

Step 3  
Define a recursive `eval` function to compute the value of an
expression. Don’t forget to handle division by zero case.

    fn eval(expr: &Expr) -> Result<i64, EvalError> {
        todo!(),
    }

Step 4  
Define function for parsing a string into an `Expr`:

    enum ParseError {

    }

    fn parse(input: &str) -> Result<Expr, ParseError> {
        todo!()
    }

Step 5  
Implement the `parse` function. It should work like this:

1.  Create a variable to hold the **stack** of expressions. In Rust, the
    appropriate type for holding a stack is a
    [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)

2.  Split the input string on whitespace
    ([`split_ascii_whitespace`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace))

3.  For each "`word`" of the input string, classify it as either one of
    the operators or as a number (`match` will be useful here).

4.  If the word is a number, *push* an expression representing the
    number to the stack (use the
    [`parse`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse)
    function to convert strings into integers).

5.  If the word is an operator, *pop* one (for `sqr`) or two (for `+`,
    `-`, `*`, `/`) operands from the stack, use them to create a
    compound `Expr`, and *push* the result back onto the stack.

6.  Don’t forget to handle error conditions:

    -   unexpected token

    -   pop from an empty stack

    -   more than one value on the stack after the end of input

Here’s a basic skeleton of the `parse` function:

    fn parse(input: &str) -> Result<Expr, ParseError> {
        let mut stack: Vec<Expr> = Vec::new();
        for word in input.split_ascii_whitespace() {

        };
        assert_eq!(stack.len(), 1);
        let res = stack.pop().unwrap();
        Ok(res)
    }

Step 6  
Check that a smoke test works:

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }

Idiomatic API
----

Now that all the logic is in place, let’s wrap it into idiomatic Rust:

1.  Implement
    [`FromStr`](https://doc.rust-lang.org/stable/std/str/trait.FromStr.html)
    for `Expr`.

2.  Implement `std::error::Error` for `ParseError` and `EvalError`.

3.  Add `enum ParseOrEvalError`, which implements `Error`,
    `From<ParseError>`, `From<EvalError>`.

4.  Add top-level
    `fn eval_str(s: &str) -> Result<i64, ParseOrEvalError>` function.
    Hint: use `?` — that’s why we’ve added `From` impls!

5.  Make `eval` a method of `Expr`.

6.  Run `clippy` on the codebase.

7.  Run `rustfmt` on the codebase.

Modularization
----

1.  Add a binary with an empty `fn main() {}` to `src/main.rs`.

2.  Make sure you can run the binary with `cargo run`.

3.  In the `main`, read a line of text from standard input. See [the
    docs](https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.read_line)
    for an example.

4.  Use `eval_str` function from the library crate to evaluate it and
    print the result. You’ll need to import the function into `main.rs`
    with `use calc::eval_str`.

5.  Modify the `main` to implement a read-eval-print loop:

        loop {
            let mut buf = String::new();
            let n_bytes_read = io::stdin().read_line(&mut buf)?;
            if n_bytes_read == 0 {
                break;
            }
            // Evaluate `buf` and print the result
        }

6.  Try building the library and the binary crate without `cargo`, using
    only `rustc`:

    $ rustc src/lib.rs --edition=2018 --crate-type rlib -o libcalc.rlib
    $ rustc src/main.rs --edition=2018 --extern=calc=./libcalc.rlib $
    ./main&lt;/programlisting&gt;

7.  Modularize the library itself: move parsing code to `mod parse;` and
    evaluation code to `mod eval;`

Naive Multithreading
----

In this task, we’ll offload expression evaluation and parsing to a
separate thread, to not block the main loop.

1.  Use
    [`std::thread::spawn`](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html)
    function in the main loop to do evaluation and printing on the other
    thread. Note that you’ll need a `move ||` closure to move the string
    buffer into a separate thread.

2.  Modify the code to also print the total sum of all the evaluated
    expressions. To do this, you’ll need a bit of state shared between
    all of the threads. You can share state between the threads using an
    `Arc`. To modify the counter, you can wrap it into a `Mutex` to
    allow for concurrent modification:

        use std::sync::{Arc, Mutex};

        let counter: Arc<Mutex<i64>> = Default::default();
        {
            let mut counter_guard = counter.lock().unwrap();
            *counter_guard += 1;
        }

3.  Replace the Mutex with lock-free `std::sync::atomic::AtomicI64`. To
    modify the counter, use `fetch_add`

        use std::sync::{Arc, atomic::{AtomicI64, Ordering}};
        let counter: Arc<AtomicI64> = Default::default();
        counter.fetch_add(1, Ordering::SeqCst);

Bonus Task
----

If you are feeling really adventurous, you can change the syntax to use
infix notation by using [this
tutorial](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).
