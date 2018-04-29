fn main() {
    let pair = (1, 1);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("`First` is 0, `y` is {:?}", y),
        (x, 0) => println!("`x` is {:?} and last is 0", x),
        // `_` means don't bind the value to a variables
        _ => println!("It does not matter what they are"),
    }
}
