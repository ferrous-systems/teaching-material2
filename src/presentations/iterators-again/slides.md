[Table of Contents](./index.html)

Returning Generic Iterators
===========================

Currently, Rust allows no generic return values. You have to use impl
trait here.

    fn main() {
        let v = vec![1,2,3];
        let i = make_iter(&v);
    }

    fn make_iter<'a>(v: &'a Vec<u8>) -> impl Iterator<Item=&u8> {
        v.iter()
    }
