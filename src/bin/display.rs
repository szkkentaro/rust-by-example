use std::fmt;

struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} +{}i", self.real, self.imag)
    }
}

fn main() {
    // 3
    println!("{}", Structure(3));

    // MinMax(10, 10)
    println!("{:?}", MinMax(10, 10));
    // (10, 10)
    println!("{}", MinMax(10, 10));

    // Point2D { x: 10.5, y: 3.4 }
    println!("{:?}", Point2D { x: 10.5, y: 3.4 });
    // { x: 10.5, y: 3.4 }
    println!("{}", Point2D { x: 10.5, y: 3.4 });

    // Complex { real: 3.3, imag: 7.2 }
    println!(
        "{:?}",
        Complex {
            real: 3.3,
            imag: 7.2
        }
    );
    // 3.3 +7.2i
    println!(
        "{}",
        Complex {
            real: 3.3,
            imag: 7.2
        }
    );
}
