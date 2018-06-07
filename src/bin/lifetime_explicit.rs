fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {}, y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // Error, `_x` does not live long enough
    // let y: &'a i32 = &_x;
    // borrowed value only lives until here
}

fn main() {
    let (four, nine) = (4, 9);

    // the lifetime of `four` and `nine` must be longer than that of `print_refs`
    print_refs(&four, &nine);

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer thant the lifetime of the function, but `'a` is longer.

    // Because the lifetime is never constrained, it defaults to `'static`.
}
