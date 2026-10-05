// Read ../../README.md before starting.

fn takes_ownership(s: String) {
    println!("got: {}", s);
}

fn makes_copy(n: i32) {
    println!("copy of n: {}", n);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let n = 42;
    makes_copy(n);
    println!("n is still: {}", n);

    let s2 = String::from("world");
    let s3 = s2;
    // println!("{}", s2);
    println!("s3: {}", s3);

    let s4 = String::from("rust");
    let s5 = s4.clone();
    println!("s4: {}, s5: {}", s4, s5);
}
