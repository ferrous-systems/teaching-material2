In this exercise we will implement a Rust-y, simpler variant of [Pig
Latin](https://en.wikipedia.org/wiki/Pig_Latin)

You will learn:

-   How to create a Rust library

-   How to deal with `Strings` and splitting

-   How to deal with `char` of a `String`

-   An early taste of Iterators

-   How to define Globals

-   Arrays vs Vectors

-   See Rust compiler’s type inference in action

-   Most common way to do string concatenation

Specification
----

For this exercise we define

-   the Vowels of English alphabet → `['a', 'e', 'i', 'o', 'u']`

-   a sentence is a collection of ASCII characters with words that are
    separated by a white space

For any given sentence, you have to modify each word of the sentence
using the following logic:

-   If the word begins with a vowel → add prefix “sr” to the word

-   If the word does not begin with a vowel → add suffix “rs” to the
    word

Tasks
----

Step 1
------

Create a new `lib` and name it `rustlatin`.

    cargo new --lib rustlatin

Step 2
------

Create a global variable in the file that defines the Vowels as
specified above

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

Step 3
------

Copy paste this skeleton of the function

    fn rustlatin(sentence: String) -> String {
        unimplemented!()
    }

    fn rustlatin(sentence: &str) -> String {
        let mut new_words = Vec::new();
        for word in sentence.split(' ') {
            let first_char_of_word = word.chars().next().unwrap();
            if VOWELS.contains(&first_char_of_word) {
                new_words.push("sr".to_string() + word);
            } else {
                new_words.push(word.to_string() + "rs");
            }
        }

        new_words.join(" ")
    }

Step 4
------

Add tests

    #[test]
    fn correct_translation() {
        // Why can we compare `&str` and `String` here?
        // https://doc.rust-lang.org/stable/std/string/struct.String.html#impl-PartialEq%3C%26%27a%20str%3E
        assert_eq!(
            "rustrs helpsrs yours sravoid sra lotrs srof srirritating bugsrs",
            rustlatin("rust helps you avoid a lot of irritating bugs")
        )
    }

    #[test]
    fn incorrect() {
        assert_ne!(
            "this shouldrs not workrs",
            rustlatin("this should not work")
        )
    }

Step 5 (optional)
-----------------

If not already done, use functional techniques (i.e. methods on
[iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)) to
write the same function. Test this new function as well.

    fn rustlatin_match(sentence: &str) -> String {
        // transform incoming words vector to rustlatined outgoing
        let new_words: Vec<_> = sentence
            .split(' ')
            .into_iter()
            .map(|word| {
                let first_char_of_word = word.chars().next().unwrap();
                if VOWELS.contains(&first_char_of_word) {
                    "sr".to_string() + word
                } else {
                    word.to_string() + "rs"
                }
            })
            .collect();

        new_words.join(" ")
    }
