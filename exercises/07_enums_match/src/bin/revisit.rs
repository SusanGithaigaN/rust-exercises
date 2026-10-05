// Read ../../README.md before starting.

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: &Shape) -> f64 {
    todo!()
}

fn describe(shape: &Shape) -> String {
    todo!()
}

fn is_valid(shape: &Shape) -> bool {
    todo!()
}

fn main() {
    let c = Shape::Circle(3.0);
    println!("{}, area: {:.2}", describe(&c), area(&c));

    let r = Shape::Rectangle(4.0, 6.0);
    println!("{}, area: {}", describe(&r), area(&r));

    let t = Shape::Triangle(3.0, 4.0, 5.0);
    println!(
        "{}, area: {}, valid: {}",
        describe(&t),
        area(&t),
        is_valid(&t)
    );

    println!("{}", is_valid(&Shape::Triangle(1.0, 2.0, 10.0)));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
