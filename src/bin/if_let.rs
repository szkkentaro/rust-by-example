fn main() {
    // All have type of Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` destructures `number` into `Some(i)`, evaluate the block (`{}`)
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // If you need the specifined failure, use an else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Did not match a number. Let's go with a letter");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Did not match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon!");
    }
}
