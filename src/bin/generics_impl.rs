struct Val {
    val: f64,
}
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

struct GenVal<T> {
    gen_val: T,
}
// Implementation requires care to remain generic
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}
