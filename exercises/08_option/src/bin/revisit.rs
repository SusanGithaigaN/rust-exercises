// Read ../../README.md before starting.

fn parse_age(s: &str) -> Option<u32> {
    todo!()
}

fn greet(name: Option<&str>) -> String {
    todo!()
}

fn find_matching<'a>(v: &'a [String], prefix: &str) -> Option<&'a String> {
    todo!()
}

fn main() {
    println!("{:?}", parse_age("25"));
    println!("{:?}", parse_age("abc"));
    println!("{:?}", parse_age("200"));
    println!("{}", greet(Some("Alice")));
    println!("{}", greet(None));

    let words = vec![
        String::from("rust"),
        String::from("ruby"),
        String::from("go"),
    ];
    println!("{:?}", find_matching(&words, "ru"));
    println!("{:?}", find_matching(&words, "py"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
