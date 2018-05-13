// Bind path to another path
use deeply::nested::function as other_func;

fn function() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

fn main() {
    // Easy access
    other_func();

    println!("Entering block");
    {
        use deeply::nested::function;
        // shadowing of `funtion()` is only in this scape.
        function();
        println!("Leaving block");
    }
    function();
}
