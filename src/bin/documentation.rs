// Documentation
#![crate_name = "doc"]

// execute
// rustc src/bin/documentation.rs --crate-type lib
// rustdoc --test --extern doc=src/lib/libdoc.rlib src/bin/documentation.rs

pub struct Person {
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    pub fn hello(&self) {
        println!("Hellom {}!", self.name);
    }
}

#[allow(dead_code)]
fn main() {
    let john = Person::new("John");
    john.hello();
}
