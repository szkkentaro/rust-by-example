fn main() {
    let mut num = 0;
    let width = 2;
    while num < 100 {
        if num % 15 == 0 {
            println!("{n:>0w$}: fizzbuzz", n = num, w = width);
        } else if num % 3 == 0 {
            println!("{n:>0w$}: fizz", n = num, w = width);
        } else if num % 5 == 0 {
            println!("{n:>0w$}: buzz", n = num, w = width);
        } else {
            println!("{n:>0w$}:", n = num, w = width);
        }
        num += 1;
    }
}
