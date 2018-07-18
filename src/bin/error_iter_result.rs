fn main() {
    let strings = vec!["tofu", "1"];
    let possible_numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", possible_numbers);
}
