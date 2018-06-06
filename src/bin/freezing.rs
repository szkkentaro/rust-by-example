fn main() {
    let mut mutable_integer = 7_i32;

    {
        let _large_integer = &mutable_integer;
        // This is error, because `murable_integer` is frozen in this scope.
        // _mutable_integer = 50;

        // `_large_integer` goes out of scope
    }

    mutable_integer = 3;
    println!("mutable_integer is {}", mutable_integer);
}
