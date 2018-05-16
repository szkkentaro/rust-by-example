#[cfg(foo)]
fn foo() {
    // rustc --cfg foo src/bin/custom_cfg.rs
    println!("custom cfg is passed");
}

#[cfg(not(foo))]
fn foo() {
    println!("custom cfg is not passed");
}

fn main() {
    foo();
}
