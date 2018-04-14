use std::mem;

fn analize_slice(slice: &[i32]) {
    println!("\tFirst element of the slice: {}", slice[0]);
    println!("\tThe slice has {} elements", slice.len());
}

fn main() {
    // Fixed-type array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    println!("The first element of the array: {}", xs[0]);
    println!("The second element of the array: {}", xs[1]);

    println!("The array length: {}", xs.len());

    // Arrays are stack allocated
    println!("The array occupies {} bytes\n", mem::size_of_val(&xs));

    println!("Borrow the whole array as slice");
    analize_slice(&xs);

    println!("Borrow a section of the array as slice");
    analize_slice(&ys[1..4]);

    // Out of bound indexing yields a panic
    println!("{}", xs[5])
}
