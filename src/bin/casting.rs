#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    // Error
    // let integer: u8 = decimal;

    // Expilicit conversion
    let integer = decimal as u8;
    let character = integer as char;
    // When casting any value to an unsigned type T,
    // std::T::Max + 1 is added or substract until the value fits into the new type
    println!("{} -> {} -> {}", decimal, integer, character);

    // 1000 alreay fits in a u16
    println!("1000 as a u16 is {}:", 1000 as u16);

    // Under the food, the first 8 lastest significant bits (LSB) are kept,
    // while the rest the most significant bits (MSB) get truncated
    // So, Below prints `232` (1000 - 256 - 256 - 256)
    println!("1000 as a u8 is {}:", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is {}:", (-1i8) as u8);

    //For positive numbers, this is same as the modules
    // prints 232
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the bitwise result is the same as first casting to the collesponding unsigned type.
    // If the most significant bit of that value is 1, then the value is negative.

    // Unless it already fits
    println!("128 as a i16 is {}:", 128 as i16);
    // 128 as u8 -> 128, whoes two's complement in eight bits is:
    println!("128 as a i8 is {}:", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is {}:", 1000 as u8);
    // and the two's complement of 232 is -24
    println!("232 as a i8 is {}:", 232 as i8);
}
