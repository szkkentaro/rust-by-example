fn main() {
    let num = 11;
    match num {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        13...19 => println!("Teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binaly = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binaly);
}
