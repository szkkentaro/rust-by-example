enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        // this will print nothing
        println!("b is foobar");
    }
    if let Foo::Qux(num) = c {
        println!("c is {}", num);
    }
}
