fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let _y = x;

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;

    // This function  takes ownership of the heap allocated memory from `b`
    destroy_box(b);
}
