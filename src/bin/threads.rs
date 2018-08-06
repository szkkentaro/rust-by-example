// Threads

use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Sign up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
