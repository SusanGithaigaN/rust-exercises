// TOPIC 08 — Option<T>
// EXERCISE TYPE: Repair
// =============================================
// Fix the Option-handling errors.
// Some of these are logic errors (will panic at runtime), not just compile errors.
// A note marks which ones panic.
//
// Run: cargo run --bin repair

fn find_first_negative(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x < 0 {
            return Some(x);
        }
    }
    None
}

fn main() {
    // ERROR 1 (runtime panic): .unwrap() on a None value
    let empty: Vec<i32> = vec![];
    let first = find_first_negative(&empty).unwrap();
    println!("first negative: {}", first);

    // ERROR 2: comparing Option values with == against a raw value (type mismatch)
    let maybe: Option<i32> = Some(5);
    if maybe == 5 {
        println!("it's five");
    }

    // ERROR 3: trying to use the inner value without unwrapping
    let x: Option<i32> = Some(42);
    let doubled = x * 2;
    println!("doubled: {}", doubled);

    // ERROR 4: match is not exhaustive — None case is missing
    let name: Option<&str> = Some("alice");
    match name {
        Some(n) => println!("hello, {}", n),
    }
}
