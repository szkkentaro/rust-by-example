// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a refetence to `NUM` where its `'static`
// lifetime is corerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a string literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }
    {
        // Make an interger to use for `coerce_static`:
        let lifetime_num = 9;
        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("Coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
