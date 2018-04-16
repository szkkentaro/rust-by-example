#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// Add a function rect_area which calculates the area of a rectangle (try using nested destructuring).
fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: p1_x, y: p1_y },
        p2: Point { x: p2_x, y: p2_y },
    } = rect;
    println!("{} {} {} {}", p1_x, p1_y, p2_x, p2_y);
    (p1_x - p2_x).abs() * (p1_y - p2_y).abs()
}

// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its lower left corner on the point, and a width and height corresponding to the f32.
// fn square() {}

// A unit struct
struct Nil;

struct Pair(i32, f32);

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("new point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    println!("destucutered values : ({}, {})", my_x, my_y);
    let rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };
    println!("{:?}", rectangle);

    // Instantiate a unit struct
    let _nil = Nil;
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("pair containts {:?} and {:?}", pair.0, pair.1);
    // Destructure the pair
    let Pair(integer, decimal) = pair;
    println!("destucutered values : ({}, {})", integer, decimal);

    // Activity
    let rect = Rectangle {
        p1: Point { x: 4.5, y: 4.5 },
        p2: Point { x: 2.5, y: 2.5 },
    };
    println!("{}", rect_area(rect));
}
