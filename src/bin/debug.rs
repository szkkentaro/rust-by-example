#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 12 months in a year
    println!("{:?} months in a year", 12);

    // "Christian" "Slater" is the "actor\'s" name.
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // Structure(3)
    println!("{:?}", Structure(3));

    /*
    Deep(
        Structure(
            3
        )
    )
    */
    println!("{:#?}", Deep(Structure(3)));
}
