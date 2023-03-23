Task
----

Take the code snippet at the end of this sheet and complete it where
indicated with `✅ TODO`.

When you’re done, it should print the following output:

    found: hay
    found: hay
    found: hay
    found: needle
    found: hay
    found: hay
    top of the haystack: hay
    look, I found the needle: ["needle"]
    bale size: 5
    empty haystack: [(), (), (), (), ()]

Code snippet to be completed
----

    // NOTE: Once you're done, your ouput could look like this:
    //
    // found: hay
    // found: hay
    // found: hay
    // found: needle
    // found: hay
    // found: hay
    // top of the haystack: hay
    // look, I found the needle: ["needle"]
    // emoji haystack: ["🌾", "🌾", "🌾", "🌾", "🌾"]

    fn main(){
        // RECAP: ANATOMY OF A CLOSURE
        //============================
        //
        let outside = "it's raining!";
        let rummage = | element | { // 👈 input parameters MAY omit type annotation
                                    // if the type can be inferred.

            println!("found: {}", element);
            println!("meanwhile, the weather: {}", outside); //  👈 closures can
            // refer to ("capture") the surrounding environment.
            // *By default*, the environment is borrowed (shared or mutable).
            // When given a choice, this is the behaviour picked by the compiler: shared > mutable > owned.
            // For further details, see https://doc.rust-lang.org/reference/types/closure.html#capture-modes

            // 👈  no return statement: just like functions or blocks, closures *can* return values,
            // but this one doesn't - hence it implictly returns the empty tuple `()`.
        };  // 👈  {}s are only needed for multi-line closures

        let haystack = vec!["hay", "hay", "hay", "needle", "hay", "hay"];

        // 👀  Closures can be used as function arguments.
        haystack.iter().for_each(rummage);

        // since we captured our environment in a shared (immutable) fashion, we're free to run this closure again:
        haystack.iter().for_each(rummage);


        // MUTATING CLOSURES
        //==================
        // Closures can mutate the variables they are capturing

        // ✅ TODO: remove all the hay from `haystack` by checking whether `key` is a needle
        // ✅ TODO: as a side effect, count the hay
        let mut haystack_clone = haystack.clone();
        let mut hay_count = 0;
        haystack_clone.retain(|key| /* check key and increment hay count here */ );
        println!("look, I found the amid between {} pieces of hay: {:?}", hay_count, haystack_clone);

        // 👀  a common use case for closures is to transform collections
        //     using e.g. `map()` and `filter()`.

        // ✅ TODO: use `map()` to convert every "hay" in the haystack to a "🌾"
        let emoji_haystack: Vec<_> = haystack
            .into_iter()
            .filter(|element | *element == "hay")
            .map( /* increment bale size here */ )
            .collect();

        println!("emoji haystack: {:?}", emoji_haystack);

        // ✅ TODO: try uncommenting  the next line. What happens when you re-compile and why?
        // println!("haystack: {:?}", haystack );

        // ✅  Bonus Task: re-implement the creation of `emoji_haystack` using `filter_map()`
        //     https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
    }

Note for Instructors
----

Distribute the code snippet below in a
[playground](https://play.rust-lang.org)

You can find an example solution in
[teaching-material/assignments/solutions/fill\_in\_the\_blanks](https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/fill_in_the_blanks).
It is called `closures.rs`. You can run it by calling
`cargo run --bin closures` in the `fill_in_the_blanks` directory.
