// Read ../../README.md before starting.

fn sum_to(n: u32) -> u32 {
    todo!()
}

fn main() {
    println!("{}", sum_to(5));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/loops.rs"));
