// `F` must implement `Fn` for a closure which takes no inputs and returns nothing
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;
    // Capture `x` into a anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);
    apply(print);
}
