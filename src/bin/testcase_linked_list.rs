use List::*;

enum List {
    // Cons: Tuple struct that wrapped an element and  a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        // `Nil` has type List
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method depends on the variant of `self`
        // `self` has type `&List`, `*self` has type `List`, matching on a concrete type `T` is prefered over a match on a reference of `&T`
        match *self {
            // Can not take ownership of tail, because self is borrowd;
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{} {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
