// macro_rules!

// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}

// Why are macros useful?
// 1. DRY
// 2. Domain-specific languages
// 3. Variadic interface
