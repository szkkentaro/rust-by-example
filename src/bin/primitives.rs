#[allow(unused_variables)]
fn main() {
    // Variable can be type annotated
    let logical: bool = true;

    // Regular annotation
    let a_float: f64 = 0.123;

    // Suffix annotation
    let a_integer = 5i32;

    // Default type is used
    let default_float = 3.14;
    let default_integer = 10;

    // A type can be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred another line
    inferred_type = 4294967296i64;

    // Mutable variables
    let mut mutable = 12; // Mutable i32
    mutable = 24;

    // Variables can be overwritten with shadowing
    let mutable = true;
}
