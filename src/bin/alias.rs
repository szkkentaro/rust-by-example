struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!(
            "Point has coordinates: ({},{},{})",
            borrowed_point.x, another_borrow.y, point.z
        );

        // This is error, because `point` has currently borrowed as immutable.
        // let mutable_borrow = &mut point;

        // Immutable references go out of scope
    }

    {
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // This is error
        // let y = &point.y;

        println!(
            "Point has coordinates: ({},{},{})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );

        // Mutable reference goes out of scope
    }

    let borrowed_point = &point;
    println!(
        "Point now has coordinates: ({},{},{})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}
