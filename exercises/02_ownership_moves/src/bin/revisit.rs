// Read ../../README.md before starting.

fn create_entry(timestamp: String, message: String) -> String {
    todo!()
}

fn prefix_entry(prefix: String, entry: String) -> String {
    todo!()
}

fn main() {
    let ts = String::from("2024-01-01");
    let msg = String::from("Server started");
    let entry = create_entry(ts, msg);
    let result = prefix_entry(String::from("INFO"), entry);
    println!("{}", result);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
