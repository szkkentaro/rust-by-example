mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called my::functino()");
}

fn private_functino() {
    println!("called my::private_functino()");
}

pub fn indirect_access() {
    print!("called my::indirect_access(), that\n> ");
    private_functino();
}
