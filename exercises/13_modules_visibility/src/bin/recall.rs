// Read ../../README.md before starting.

mod geometry {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            todo!()
        }

        pub fn distance(&self, other: &Point) -> f64 {
            todo!()
        }
    }

    pub mod shapes {
        use super::Point;

        pub struct Circle {
            pub center: Point,
            pub radius: f64,
        }

        impl Circle {
            pub fn new(center: Point, radius: f64) -> Circle {
                todo!()
            }

            pub fn area(&self) -> f64 {
                todo!()
            }

            pub fn contains(&self, p: &Point) -> bool {
                todo!()
            }
        }
    }
}

use geometry::shapes::Circle;
use geometry::Point;

fn main() {
    let origin = Point::new(0.0, 0.0);
    let far = Point::new(3.0, 4.0);
    println!("{}", origin.distance(&far));

    let c = Circle::new(Point::new(0.0, 0.0), 10.0);
    println!("{}", c.area());
    println!("{}", c.contains(&Point::new(3.0, 4.0)));
    println!("{}", c.contains(&Point::new(10.0, 10.0)));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
