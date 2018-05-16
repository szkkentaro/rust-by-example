#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

fn main() {
    are_you_on_linux();
    println!("Are you sure?");

    // cfg macro
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitly linux.");
    } else if cfg!(target_os = "macos") {
        println!("Yes. It's definitly **not** linux. It's macos.");
    } else {
        println!("Yes. It's definitly **not** linux.");
    }
}
