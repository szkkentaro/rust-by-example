#![crate_type = "lib"]
#![crate_name = "foo"]

pub fn public_function() {
    println!("called rary's function()");
}
fn private_function() {
    println!("called rary's private_function()");
}
pub fn indirect_access() {
    print!("called rary's indirect_access(), that\n> ");
    private_function();
}
