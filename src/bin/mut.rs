fn main() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // This is error
    // *immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}
