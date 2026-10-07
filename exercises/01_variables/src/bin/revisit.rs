// Read ../../README.md before starting.

fn bmi_category(weight_kg: f64, height_m: f64) -> &'static str {
    
}

fn describe_temp(celsius: f64) -> &'static str {
    todo!()
}

fn main() {
    println!("{}", bmi_category(70.0, 1.75));
    println!("{}", bmi_category(50.0, 1.75));
    println!("{}", bmi_category(100.0, 1.75));
    println!("{}", describe_temp(-5.0));
    println!("{}", describe_temp(10.0));
    println!("{}", describe_temp(30.0));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
