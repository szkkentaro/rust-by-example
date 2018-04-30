// Derefence uses `*`
// Destructuring uses `&`, `ref`, and `ref mut`
fn main() {
    let reference = &4;
    match reference {
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
