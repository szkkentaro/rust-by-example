fn main() {
    // the compiler knows that `elem` has type u8.
    let elem = 5u8;
    // Create an empty vector, however at this point the compiler does not know the exact type of `vec`,
    // it just knows that its a vector of something (like Vec<_>)
    let mut vec = Vec::new();

    // Push elem in vector, Now the compiler knows that `vec` has type Vec<u8>
    vec.push(elem);
    println!("{:?}", vec);
}
