// Read ../../README.md before starting.

mod geometry;

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
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
