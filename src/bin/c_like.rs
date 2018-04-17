#![allow(dead_code)]

// Enum with implicite discriminator
enum Number {
    Zero,
    One,
    Two,
}

// Enum with explicite discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // enums can be cast as integer
    println!("{}", Number::Zero as i32);
    println!("{}", Number::One as i32);

    println!("{:06x}", Color::Red as i32);
    println!("{:06x}", Color::Green as i32);
}
