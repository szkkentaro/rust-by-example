// Iterators

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returnd.
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calles on 0.3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The `next four terms of the Fibonacci sequence are:");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    // The `iter` method produces an `Iterator` over an array/slice.
    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
