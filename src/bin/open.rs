// open

// echo "Hello" > hello.txt && cargo run --bin open && rm hello.txt

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    // returns `io::Result<usise>`
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read{}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
