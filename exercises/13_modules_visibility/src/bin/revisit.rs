// Read ../../README.md before starting.

mod stats {
    // Private helper: return a sorted copy of v (smallest first).
    fn sorted_copy(v: &[f64]) -> Vec<f64> {
        todo!()
    }

    // Arithmetic mean. None if v is empty.
    pub fn mean(v: &[f64]) -> Option<f64> {
        todo!()
    }

    // Middle value of sorted v. Two-element middle → average the pair. None if empty.
    pub fn median(v: &[f64]) -> Option<f64> {
        todo!()
    }

    // Most frequent value. Ties: return the smallest. None if empty.
    pub fn mode(v: &[i32]) -> Option<i32> {
        todo!()
    }
}

fn main() {
    println!("{:?}", stats::mean(&[1.0, 2.0, 3.0, 4.0, 5.0]));   // Some(3.0)
    println!("{:?}", stats::median(&[1.0, 2.0, 3.0, 4.0, 5.0])); // Some(3.0)
    println!("{:?}", stats::median(&[1.0, 2.0, 3.0, 4.0]));      // Some(2.5)
    println!("{:?}", stats::mode(&[1, 2, 2, 3, 3, 3]));          // Some(3)
    println!("{:?}", stats::mean(&[]));                           // None
    // stats::sorted_copy(&[]); // should not compile — private function
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
