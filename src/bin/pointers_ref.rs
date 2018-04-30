// Derefence uses `*`
// Destructuring uses `&`, `ref`, and `ref mut`
fn main() {
    // Assign a reference of type `i32`
    let reference = &4;
    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // &i32
        // &val
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    match value {
        ref r => println!("Got a reference to value: {:?}", r),
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We add `10`. `mut_value`: {:?}", m)
        }
    }
}
