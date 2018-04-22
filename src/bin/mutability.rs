fn main() {
    let _imutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Error
    // _imutable_binding += 1;
}
