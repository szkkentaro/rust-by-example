fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}
fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let _equivalent_floot = Some(0f32);

    let optional_floot = Some(0f32);

    println!(
        "{:?} unwraps to {:?}",
        optional_floot,
        optional_floot.unwrap()
    );

    // Unwrapping a `None` variant will `panic!`
    // let none: Option<i32> = None;
    // println!("{:?} unwraps to {:?}", none, none.unwrap());
}
