fn function() {
    println!("call function()");
}

mod cool {
    pub fn function() {
        println!("call cool::function()");
    }
}

mod my {
    fn function() {
        println!("call my::function");
    }

    mod cool {
        pub fn function() {
            println!("call my::cool::function()");
        }
    }

    pub fn indrect_call() {
        self::function();
        function();
        self::cool::function();
        super::function();
        {
            use cool::function as other_func;
            other_func();
        }
    }
}

fn main() {
    my::indrect_call();
}
