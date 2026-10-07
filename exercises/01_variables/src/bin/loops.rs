// Read ../../README.md before starting.

fn sum_to(n: u32) -> u32 {
    // let n:u32 = 1000; supposed to assume, not declare
    let mut count: u32 = 0;
    for i in 1..=n {
        // println!("{}, count");
        count += i;
    }
    count
}

fn main() {
    println!("{}", sum_to(5));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/loops.rs"));
