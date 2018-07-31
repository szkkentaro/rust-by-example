// Execute
// rustc src/bin/panic.rc
// MallocStackLogging=1 ./panic

// Re-implementation of integer division (/)
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggeres a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn main() {
    // Heap allocated integer
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    division(3, 0);

    println!("This piont won't be readched!");

    // _x should get destroyed at this point
}
