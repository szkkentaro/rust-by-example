fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (
        1u8,
        2u16,
        3u32,
        4u64,
        -1i8,
        -2i16,
        -3i32,
        -4i64,
        0.1f32,
        0.2f64,
        'a',
        true,
    );
    println!("{}", long_tuple.0);
    println!("{}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // Tuples are printable
    println!("{:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    println!("one element tuple is {:?}", (5u32,));
    println!("just an integer {:?}", (5u32));

    // Tuple can be destructured to create binding
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.2, 2.3, 3.4, 4.5);
    println!("{:?}", matrix);
}
