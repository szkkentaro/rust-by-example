fn main() {
    // General
    println!("{} days", 31);
    // Positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    // pad numbers with extra zeroes
    println!("{number:>0width$}", number = 1, width = 6);

    // sets the number of decimal places in floating-point types
    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}
