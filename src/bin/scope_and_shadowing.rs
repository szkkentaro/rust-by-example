fn main() {
    let long_living_binding = 1;
    {
        let short_living_binding = 2;
        println!("inner short: {}", short_living_binding);

        // This binding shadows the outer one
        let long_living_binding = 5_f32;
        println!("inner long: {}", long_living_binding);
    }

    // Error
    // println!("inner short: {}", short_living_binding);

    // Print `1`
    println!("outer long: {}", long_living_binding);

    // This binding shadows the previous binding
    let long_living_binding = 'a';
    println!("outer long: {}", long_living_binding);
}
