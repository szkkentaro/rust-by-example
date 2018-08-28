#![crate_type = "lib"]
#![crate_name = "try"]

// How to run
//   rustc --crate-type=lib src/lib/motivation_behind_documentation_tests.rs
//   rustdoc --test src/lib/adder.rs --extern adder=src/lib/libadder.rlib

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // rerutning from try_main()
/// # }
/// # fn main() { //starting main that'll unwrap()
/// #   try_main().unwrap(); // calling try_main and unwrapping
/// #                        // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
