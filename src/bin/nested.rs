#[allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("Enter the outer loop");
        'inner: loop {
            println!("Enter the inner loop");
            // This would break only inner loop
            // break;
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("existed the outer loop");
}
