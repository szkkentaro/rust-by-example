fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("{}, {}, {}", a, b, y);

    // order is not important
    let Foo { y: i, x: j } = foo;
    println!("{:?}, {:?}", i, j);

    // You can also ignore some variables
    let Foo { y, .. } = foo;
    println!("{:?}", y);

    // This will give an error: pattern does not mention field `x`
    // let Foo {y} = foo;
}
