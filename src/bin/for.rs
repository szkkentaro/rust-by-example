#![allow(dead_code)]
fn fizzbuz() {
    for n in 0..100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_with_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    // This borrrows each elemement of collection through each iteration.
    for name in names.iter() {
        match name {
            // Thus leaving the collection untouched and available for reuse after the loop.
            &"Ferris" => println!("There is a rustacean amoung us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_with_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    // This consumes the collection so that on each iteration extract data is provided
    for name in names.into_iter() {
        match name {
            // Once the collection has been consumed it is no longer available for resuse as it has been `moved` with in a loop
            "Ferris" => println!("There is a rustacean amoung us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_with_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    // This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean amoung us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn main() {
    // Fizzbuzz instead of loop syntax
    // fizzbuz();

    for_with_iter();
    for_with_into_iter();
    for_with_iter_mut();
}
