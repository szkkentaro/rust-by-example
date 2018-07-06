// Early returns

use std::num::ParseIntError;

fn multiply(first_number_str: &str, seconde_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        // Early return
        Err(e) => return Err(e),
    };
    let second_number = match seconde_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        // Early return
        Err(e) => return Err(e),
    };
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
