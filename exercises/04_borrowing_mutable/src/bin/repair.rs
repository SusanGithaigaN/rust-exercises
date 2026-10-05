// TOPIC 04 — Borrowing: Mutable References
// EXERCISE TYPE: Repair
// =============================================
// Fix the borrow rule violations.
// Read each error — the compiler will tell you which rule is broken.
//
// Run: cargo run --bin repair

fn main() {
    // ERROR 1: two mutable borrows of the same value at the same time
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{} {}", r1, r2);

    // ERROR 2: a mutable borrow while a shared borrow is still active
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("first: {}", first);

    // ERROR 3: trying to mutate through a shared reference (not mut)
    let mut n = 10;
    let r = &n;
    *r = 20;
    println!("{}", n);
}
