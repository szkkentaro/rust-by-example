#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}
#[derive(Debug)]
enum Work {
    Clivilian,
    Soldier,
}

fn main() {
    // Expilicitly `use` each name so ther are available without manual scoping.
    use Status::{Poor, Rich};
    // Automaticaly `use` each name inside `Work`
    use Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Work::Soldier`
    let work = Clivilian;
    match status {
        Rich => println!("{:?}", Rich),
        Poor => println!("{:?}", Poor),
    }
    match work {
        Clivilian => println!("{:?}", Clivilian),
        Soldier => println!("{:?}", Soldier),
    }
}
