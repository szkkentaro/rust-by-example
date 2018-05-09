fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`
    let mut into_iter = vec2.into_iter();
    // A reference what is yeilded is `&&i32`. Destructure to `i32`
    println!("Find in vec1: {:?}", iter.find(|&&x| x == 2));
    // A reference what is yeilded is `&i32`. Destructure to `i32`
    println!("Find in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // A reference what is yeilded is `&i32`. Destructure to `i32`
    println!(
        "Find in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );
}
