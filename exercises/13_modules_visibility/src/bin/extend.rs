// TOPIC 13 — Modules and Visibility
// EXERCISE TYPE: Extend
// =============================================
// The geometry module from recall is provided inline.
// Your task: restructure it into separate files.
//
// Current structure (everything in one file):
//   mod geometry { ... pub mod shapes { ... } }
//
// New structure (split across files):
//   src/bin/extend.rs        <- this file (main only, with mod declarations)
//   src/geometry.rs          <- geometry module (Point + distance)
//   src/geometry/shapes.rs   <- shapes submodule (Circle + methods)
//
// Steps:
//   1. Create src/geometry.rs — move Point and its impl there.
//      Add `pub mod shapes;` inside it.
//   2. Create src/geometry/shapes.rs — move Circle and its impl there.
//      Add `use super::Point;` at the top.
//   3. In this file (extend.rs), declare `mod geometry;` and use the types.
//
// NOTE: This is a file system exercise. You will need to create new files.
//       The compiler error messages will guide you if paths are wrong.
//
// Expected output (same as recall):
//   distance: 5
//   circle area: 78.53981633974483
//   (3,4) inside circle: true
//   (10,10) inside circle: false
//
// Run: cargo run --bin extend

// TODO: replace this inline module with `mod geometry;` after creating the files

mod geometry {
    pub struct Point { pub x: f64, pub y: f64 }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point { Point { x, y } }
        pub fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    pub mod shapes {
        use super::Point;
        pub struct Circle { pub center: Point, pub radius: f64 }

        impl Circle {
            pub fn new(center: Point, radius: f64) -> Circle { Circle { center, radius } }
            pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
            pub fn contains(&self, p: &Point) -> bool { self.center.distance(p) < self.radius }
        }
    }
}

use geometry::Point;
use geometry::shapes::Circle;

fn main() {
    let origin = Point::new(0.0, 0.0);
    let far = Point::new(3.0, 4.0);
    println!("distance: {}", origin.distance(&far));

    let circle = Circle::new(Point::new(0.0, 0.0), 10.0);
    println!("circle area: {}", circle.area());
    println!("(3,4) inside circle: {}", circle.contains(&Point::new(3.0, 4.0)));
    println!("(10,10) inside circle: {}", circle.contains(&Point::new(10.0, 10.0)));
}
