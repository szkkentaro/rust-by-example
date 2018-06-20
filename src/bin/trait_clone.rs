#[derive(Debug, Clone, Copy)]
struct Nil;
#[derive(Debug, Clone)]
struct Pair(Box<u32>, Box<u32>);

fn main() {
    let nil = Nil;
    let copied_nil = nil;
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error `pair` has lost its resources
    // println!("origin: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // `moved_pair` has been dropped
    // println!("moved: {:?}", moved_pair);

    println!("clone: {:?}", cloned_pair);
}
