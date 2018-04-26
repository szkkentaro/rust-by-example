use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "Hello";
    let my_string = String::from(my_str);
    println!("{}", my_string);

    let number = Number::from(30);
    println!("{:?}", number);

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("my number is {:?}:", num);
}
