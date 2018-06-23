// Dseignators

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`
    // The `ident` designator is used for variable/function names
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    foo();
    bar();
    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
