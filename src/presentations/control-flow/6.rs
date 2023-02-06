use std::fs::File;
use std::io;

fn main() {
    let file_open: Result<File, io::Error> = File::open("Does not exist");

    match file_open {
        Ok(f)  => println!("Success!"),
        Err(e) => println!("Open failed: {:?}", e),
    }
}
