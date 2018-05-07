fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_i32<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;
    let greeting = "hello";
    // A non-copy type
    // `to_owned` creates owned data from borrow one.
    let mut farewell = "goodby".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep.");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_i32(double));
}
