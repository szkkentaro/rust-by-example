// The main use of `alias` is to reduce boilerplate;
// For example, the `IoResult<T>` type is an alias for the `Result<T, IoError>` type

// `NanoSecond` is a new name for `f64`
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inchs: Inch = 2 as u64_t;
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inchs,
        nanoseconds + inchs
    );
}
