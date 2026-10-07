// Read ../../README.md before starting.

fn double(n: i32) -> i32 {
    // Bit shift method
    // let n: i32 = n <<1;
    // arithmetic operator
    let n: i32 = n * 2;
    return n;
}

fn describe_sign(n: i32) -> &'static str {
    if n > 0 {
        return "positive";
    } else if n < 0 {
        return "negative";
    } else {
        return "zero";
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // todo!()
    // 1c = 33.8 faren
    let f: f64 = 33.8;
    let result: f64 = c * f;
    return result;

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
