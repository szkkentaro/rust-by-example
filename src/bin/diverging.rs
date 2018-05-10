#![feature(never_type)]

#[allow(dead_code)]
fn foo() -> ! {
    panic!("This call never returns");
}

fn some_fn() {
    ()
}

#[allow(unused_variables, unreachable_code)]
fn main() {
    let a: () = some_fn();
    println!("This function return , and you can see this line {:?}", a);

    // let x: ! = panic!("This call never returns");
    // println!("You will never see this line");

    fn some_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("{}", some_odd_numbers(9));
}
