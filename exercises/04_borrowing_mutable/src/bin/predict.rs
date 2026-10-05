// TOPIC 04 — Borrowing: Mutable References
// EXERCISE TYPE: Predict
// =============================================
// This file has TWO separate examples in fn example_a and fn example_b.
// Comment out one, run the other. Then swap.
//
// For each example, BEFORE running, answer:
//   1. Will it compile? Yes or No.
//   2. If No — what rule does it break?
//
// Run: cargo run --bin predict

fn example_a() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("{}", r1);

    // r1 is no longer used after this point
    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", r2);
}

fn example_b() {
    let mut s = String::from("hello");

    let r1 = &s;     // shared borrow
    let r2 = &s;     // another shared borrow
    println!("{} {}", r1, r2);

    // r1 and r2 are no longer used after the println above
    let r3 = &mut s; // mutable borrow — is this OK here?
    r3.push_str("!");
    println!("{}", r3);
}

fn main() {
    example_a();
    // example_b(); // uncomment to test the second example
}
