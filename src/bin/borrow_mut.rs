#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowd {} - {} edition", book.title, book.year);
}

// This function takes a reference to mutable Book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowd {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutable`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "夏目漱石",
        title: "吾輩は猫である",
        year: 1905,
    };

    let mut mutabook = immutabook;

    // Immutabily borrow an immutable object
    borrow_book(&immutabook);
    // Immutabily borrow a mutable object
    borrow_book(&mutabook);
    // Borrow a mutable aobject as mutable
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
}
