// Read ../../README.md before starting.

fn bmi_category(weight_kg: f64, height_m: f64) -> &'static str {
    // bmi = wieght in kg/(height *height in metres)
    let bmi = weight_kg / (height_m * height_m);

    if bmi < 18.5 {
        return "underweight";
    } else if bmi < 25 as f64 {
        return "normal";
    } else if bmi < 30 as f64 {
        return "overweight";
    } else {
        return "obese";
    }
}

fn describe_temp(celsius: f64) -> &'static str {
    // todo!()
    if celsius < 0 as f64{
        "freezing"
    } else if celsius < 15 as f64{
        "cold"
    } else if celsius <25 as f64{
        "warm"
    }else {
        "hot"
    }
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
