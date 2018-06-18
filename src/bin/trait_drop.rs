struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }

    println!("Just exited block A");

    drop(_a);

    println!("end of the main functino");

    // `_a` wont be droped again here, because it already has been (munually) `drop`ed
}
