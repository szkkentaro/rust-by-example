static LANGUAGE: &'static str = "ja";
const THRESHOLD: u8 = 10;

fn is_big(n: u8) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("LANGUAGE is {}", LANGUAGE);
    println!("THRESHOLD is {}", THRESHOLD);

    let n: u8 = 12;
    println!(
        "{} is {} than {}",
        n,
        if is_big(n) { "bigger" } else { "less" },
        THRESHOLD
    );

    // THRESHOLD = 5;
    // ^ occures error, because `const` can not be modified
}
