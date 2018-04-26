fn main() {
    let n = 11;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n <= 10 && n >= -10 {
        println!(", and is a small number, increase ten-hold");
        n * 10
    } else {
        println!(", and is a big number, half the number");
        // This expression must return an `i32` as well.
        // Try suppressing this expression with semicolon.
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
