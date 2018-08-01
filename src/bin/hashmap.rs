// HashMap

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "111-1111" => "The call cannot be completed as dialed.",
        "222-2222" => "Hello, this is Awesome Pizza",
        _ => "Hi! Who is again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Alice", "111-1111");
    contacts.insert("Bob", "222-2222");

    match contacts.get(&"Alice") {
        Some(&number) => println!("Calling Alice: {}", call(number)),
        _ => println!("Don't have Alice's number"),
    }

    contacts.insert("Alice", "333-3333");

    match contacts.get(&"Bob") {
        Some(&number) => println!("Calling Bob: {}", call(number)),
        _ => println!("Don't have Bob's number"),
    }

    contacts.remove(&"Bob");

    for (contact, &number) in contacts.iter() {
        println!("Calling {}:{}", contact, call(number));
    }
}
