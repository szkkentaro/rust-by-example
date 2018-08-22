#![crate_type = "lib"]
#![crate_name = "doccomments"]

// How to run
// 1. complie to lib
//    rustc --crate-type=lib src/lib/documentation_testing.rs
// 2. test
//    rustdoc --test src/lib/documentation_testing.rs --extern doccomments=src/lib/libdoccomments.rlib

/// Fisrt line is a short summary describing functino.
///
/// The next lines present detailed documentaion.
/// Code blocks start with triple backquotes and have implicit `fn main()` inside and `extern crate <createname>`.
/// Assume we're testing `doccomments` crate:
///
/// # Examples
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// Usualy doc comments may include sections "Examples", "Panics" and "Failures"
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the secnd argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}
