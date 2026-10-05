// Read ../../README.md before starting.

mod stats {
    fn sorted_copy(v: &[f64]) -> Vec<f64> {
        todo!()
    }

    pub fn mean(v: &[f64]) -> Option<f64> {
        todo!()
    }

    pub fn median(v: &[f64]) -> Option<f64> {
        todo!()
    }

    pub fn mode(v: &[i32]) -> Option<i32> {
        todo!()
    }
    // Supplied checks; leave these unchanged.
    #[cfg(test)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
}

fn main() {
    println!("{:?}", stats::mean(&[1.0, 2.0, 3.0, 4.0, 5.0]));
    println!("{:?}", stats::median(&[1.0, 2.0, 3.0, 4.0, 5.0]));
    println!("{:?}", stats::median(&[1.0, 2.0, 3.0, 4.0]));
    println!("{:?}", stats::mode(&[1, 2, 2, 3, 3, 3]));
    println!("{:?}", stats::mean(&[]));
}
