// panic
// RUST_BACKTRACE=1 cargo run --bin error_panic

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAAaaaa!");
    }
    println!("I love {}s!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
