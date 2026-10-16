// Read ../../README.md before starting.

mod geometry {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        // Create a Point at (x, y).
        pub fn new(x: f64, y: f64) -> Point {
            todo!()
        }

        // Euclidean distance: √((x2-x1)² + (y2-y1)²).
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
            // Create a Circle at the given center with the given radius.
            pub fn new(center: Point, radius: f64) -> Circle {
                todo!()
            }

            // π × radius².
            pub fn area(&self) -> f64 {
                todo!()
            }

            // True if p is strictly inside the circle (distance from center < radius).
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
    println!("{}", origin.distance(&far)); // 5

    let c = Circle::new(Point::new(0.0, 0.0), 10.0);
    println!("{}", c.area());                              // 78.53981633974483
    println!("{}", c.contains(&Point::new(3.0, 4.0)));    // true
    println!("{}", c.contains(&Point::new(10.0, 10.0)));  // false
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
