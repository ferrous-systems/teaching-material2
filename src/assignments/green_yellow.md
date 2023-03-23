In this exercise, you will learn

-   How to work with rust slices and vectors.

-   How to accept input from stdin.

In this assignment we will implement the game "Green and Yellow".

It’s like Wordle, but with numerical digits instead of letters. But for
legal reasons it’s also entirely unlike Wordle, nor remotely similar to
the 1970’s board-game "Mastermind".

The game creates a random secret with four random numbers from 1 to 9.

For example:

\`\`\` secret: 3 3 1 7 \`\`\`

We try to guess the secret but are given only a mysterious pattern of
coloured boxes:

-   Green boxes are correct guesses in the correct place.

-   Yellow boxes are correct guesses in the wrong place.

Values are counted exactly once in the guess and the secret.

The task is to implement this function:

\`\`\` fn calc\_green\_and\_yellow(guess: &\[i32\], secret: &\[i32\]) →
String { … } \`\`\`

This returns a seven character string - the four results for the four
digits of the guess, interspersed with space characters. e.g.:

\`\`\`text 🟩 ⬜ ⬜ 🟨 \`\`\`

Here is a test to help you:

\`\`\` \#\[test\] fn test\_green\_and\_yellow() {
assert\_eq!(calc\_green\_and\_yellow(&\[1, 2, 3, 4\], &\[1, 2, 3, 4\]),
"🟩 🟩 🟩 🟩".to\_string()); assert\_eq!(calc\_green\_and\_yellow(&\[1, 2,
3, 5\], &\[1, 2, 3, 4\]), "🟩 🟩 🟩 ⬜".to\_string());
assert\_eq!(calc\_green\_and\_yellow(&\[4, 3, 2, 1\], &\[1, 2, 3, 4\]),
"🟨 🟨 🟨 🟨".to\_string()); assert\_eq!(calc\_green\_and\_yellow(&\[1, 2,
3, 1\], &\[1, 2, 3, 4\]), "🟩 🟩 🟩 ⬜".to\_string());
assert\_eq!(calc\_green\_and\_yellow(&\[1, 1, 1, 1\], &\[1, 2, 3, 4\]),
"🟩 ⬜ ⬜ ⬜".to\_string()); assert\_eq!(calc\_green\_and\_yellow(&\[1, 2,
2, 2\], &\[2, 2, 2, 1\]), "🟨 🟩 🟩 🟨".to\_string());
assert\_eq!(calc\_green\_and\_yellow(&\[1, 3, 3, 2\], &\[2, 2, 2, 1\]),
"🟨 ⬜ ⬜ 🟨".to\_string()); } \`\`\`

Step 1.
----

Use the "rand" crate to generate a secret number.

\`\`\` let secret: Vec&lt;\_&gt; = (0..NUM\_DIGITS).map(|\_| ???
).collect(); \`\`\`

Step 2.
----

Parse the input strings (the user’s guess).

\`\`\` let stdin = std::io::stdin();

    let mut buf = String::new();

        loop {
            buf.clear();
            print!("guess: ");
            stdin.read_line(&mut buf).unwrap();
            let guess : Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();
        }
    ```

Step 3.
----

If the guess is correctly formed, return the appropriate combination of
green and yellow squares.

\`\`\` let squares = calc\_green\_and\_yellow(&guess, &secret);
println!("{:?} got {}", guess, squares); \`\`\`

Step 4.
----

Write the `calc_green_and_yellow` function.

Hint:

Check the green squares first and then tick them off by removing them
from both the guess and the secret. Then check the yellow squares. This
way you don’t count anything twice.

Make sure all the tests pass.

Step 5.
----

Play the game.
