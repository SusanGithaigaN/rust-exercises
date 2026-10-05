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

fn main() {
    println!("{}", double(5));
    println!("{}", double(-3));
    println!("{}", describe_sign(7));
    println!("{}", describe_sign(0));
    println!("{}", describe_sign(-4));
    println!("{}", celsius_to_fahrenheit(0.0));
    println!("{}", celsius_to_fahrenheit(100.0));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
