In this exercise we will implement a function that returns a `bool`
whether a number (integer) is a Narcissistic number or not.

Specification
=============

A number is Narcissistic number if the sum of each of its digits raised
to the power of the number of digits is equal to the number itself. Also
called Armstrong number.

e.g. 407 = 4<sup>3\ +\ 0</sup>3 + 7^3 = 407

[You can read more about them on
Wikipedia](https://en.wikipedia.org/wiki/Narcissistic_number)

You will learn:

-   How to create a Rust library

-   To get more practice with iterators

Tasks
=====

Step 1
------

Create a new `lib` and name it `narcheck`.

    cargo new --lib narcheck

Step 2
------

Create a function that converts an integer into its digits. The
signature of the function is -

    fn get_digits(num: u32) -> Vec<u32>

    fn get_digits(num: u32) -> Vec<u32> {
        let digits: Vec<_> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        digits
    }

Step 3
------

Create a function that uses the `get_digits` function above to check if
a number is Narcissistic number. The signature of the function is -

    fn is_narcissistic(num: u32) -> bool

    fn is_narcissistic(num: u32) -> bool {
        let digits = get_digits(num);
        let cubed_digits: Vec<u32> = digits
            .iter()
            .map(|d| u32::pow(*d, digits.len() as u32))
            .collect();
        let sum: u32 = cubed_digits.iter().sum();
        sum == num
    }

Step 4
------

Add tests.

    #[test]
    fn nar() {
        assert!(is_narcissistic(407))
    }

    #[test]
    fn not_nar() {
        assert_eq!(false, is_narcissistic(144))
    }
