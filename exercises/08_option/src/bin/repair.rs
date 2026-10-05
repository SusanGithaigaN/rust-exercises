// Read ../../README.md before starting.

fn find_first_negative(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x < 0 {
            return Some(x);
        }
    }
    None
}

fn main() {
    // ERROR 1 — runtime panic
    let empty: Vec<i32> = vec![];
    let first = find_first_negative(&empty).unwrap();
    println!("first negative: {}", first);

    // ERROR 2
    let maybe: Option<i32> = Some(5);
    if maybe == 5 {
        println!("it's five");
    }

    // ERROR 3
    let x: Option<i32> = Some(42);
    let doubled = x * 2;
    println!("doubled: {}", doubled);

    // ERROR 4
    let name: Option<&str> = Some("alice");
    match name {
        Some(n) => println!("hello, {}", n),
    }
}
