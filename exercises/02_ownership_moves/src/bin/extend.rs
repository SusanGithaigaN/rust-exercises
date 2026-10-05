// Read ../../README.md before starting.

fn append_exclamation(s: String) -> String {
    todo!()
}
fn shout(s: String) -> String {
    todo!()
}

fn double_elements(v: Vec<i32>) -> Vec<i32> {
    todo!()
}

fn sum_and_drop(v: Vec<i32>) -> i32 {
    todo!()
}

fn main() {
    let s = String::from("hello");
    println!("{}", shout(append_exclamation(s)));

    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", double_elements(v));

    let v2 = vec![1, 2, 3, 4, 5];
    println!("{}", sum_and_drop(v2));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
