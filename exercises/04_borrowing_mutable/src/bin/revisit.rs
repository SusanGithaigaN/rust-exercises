// Read ../../README.md before starting.

fn remove_duplicates(v: &mut Vec<i32>) {
    todo!()
}

fn normalize(v: &mut Vec<f64>) {
    todo!()
}

fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    remove_duplicates(&mut v);
    println!("{:?}", v);

    let mut f = vec![1.0, 2.0, 3.0];
    normalize(&mut f);
    println!("{:?}", f);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
