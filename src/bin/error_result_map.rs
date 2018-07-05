//  map for Result
use std::num::ParseIntError;

fn _multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn other_multiply(first_number_str: &str, seconde_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        seconde_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = other_multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = other_multiply("t", "2");
    print(tt);
}
