fn main() {
    let result: Option<u8> = 5_u8.checked_add(5);

    match result {
        Some(result) if result % 2 == 0 => println!("5+5 is even!"),
        _ => println!("5+5 ... isn't even?"),
    }
}
