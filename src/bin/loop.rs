fn main() {
    let mut count = 0u32;
    println!("let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, That's enough");

            // Exit loop
            break;
        }
    }
}
