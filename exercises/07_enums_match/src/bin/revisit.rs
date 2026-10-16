// Read ../../README.md before starting.

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// Circle: π×r². Rectangle: w×h. Triangle: use Heron's formula.
fn area(shape: &Shape) -> f64 {
    todo!()
}

// Return a description string, e.g. "Circle with radius 3", "Rectangle 4 x 6", "Triangle 3-4-5".
fn describe(shape: &Shape) -> String {
    todo!()
}

// Circle: radius > 0. Rectangle: both sides > 0. Triangle: sum of any two sides > third side.
fn is_valid(shape: &Shape) -> bool {
    todo!()
}

fn main() {
    let c = Shape::Circle(3.0);
    println!("{}, area: {:.2}", describe(&c), area(&c)); // Circle with radius 3, area: 28.27

    let r = Shape::Rectangle(4.0, 6.0);
    println!("{}, area: {}", describe(&r), area(&r));    // Rectangle 4 x 6, area: 24

    let t = Shape::Triangle(3.0, 4.0, 5.0);
    println!(
        "{}, area: {}, valid: {}",
        describe(&t),
        area(&t),
        is_valid(&t)
    ); // Triangle 3-4-5, area: 6, valid: true

    println!("{}", is_valid(&Shape::Triangle(1.0, 2.0, 10.0))); // false
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
