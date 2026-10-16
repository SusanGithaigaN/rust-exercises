// Read ../../README.md before starting.

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        todo!()
    }
    fn area(&self) -> f64 {
        todo!()
    }
    fn perimeter(&self) -> f64 {
        todo!()
    }
    fn is_square(&self) -> bool {
        todo!()
    }
    fn scale(&self, factor: f64) -> Rectangle {
        todo!()
    }

    // True when self is strictly wider and taller than other.
    fn can_contain(&self, other: &Rectangle) -> bool {
        todo!()
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    // Create a Circle with the given radius.
    fn new(radius: f64) -> Circle {
        todo!()
    }

    // π × radius².
    fn area(&self) -> f64 {
        todo!()
    }

    // 2 × π × radius.
    fn circumference(&self) -> f64 {
        todo!()
    }
}

fn main() {
    let big = Rectangle::new(10.0, 8.0);
    let small = Rectangle::new(5.0, 3.0);
    println!("{}", big.can_contain(&small));   // true
    println!("{}", small.can_contain(&big));   // false

    let c = Circle::new(3.0);
    println!("{}", c.area());          // 28.274333882308138
    println!("{}", c.circumference()); // 18.84955592153876
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
