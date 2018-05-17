#[derive(Debug)]
struct A;

struct Single(A);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    // `Single is a concrete and explicitly takes `A`
    let _s = Single(A);

    // create `_char` of type `SingleGen<char>`
    let _char: SingleGen<char> = SingleGen('a');

    let t = SingleGen(A); // Uses A
    let n = SingleGen(6); // Uses i32
    let a = SingleGen('A'); // Uses char
    println!("{:?}, {:?}, {:?}", t, n, a);
}
