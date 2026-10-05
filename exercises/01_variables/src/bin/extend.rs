// Read ../../README.md before starting.

fn double(n: i32) -> i32 {
    todo!()
}
fn describe_sign(n: i32) -> &'static str {
    todo!()
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

fn is_between(value: i32, low: i32, high: i32) -> bool {
    todo!()
}

fn main() {
    println!("{}", double(5));
    println!("{}", describe_sign(-3));
    println!("{}", celsius_to_fahrenheit(100.0));
    println!("{:?}", swap(3, 7));
    println!("{:?}", swap(-1, 5));
    println!("{}", is_between(5, 1, 10));
    println!("{}", is_between(11, 1, 10));
    println!("{}", is_between(1, 1, 10));
    println!("{}", is_between(0, 1, 10));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
