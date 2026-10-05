// TOPIC 02 — Ownership: Moves and Scope
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these questions in your notes:
//   1. After takes_ownership(s), can you use s? Why?
//   2. After makes_copy(n), can you use n? Why?
//   3. After let s3 = s2, what happened to s2?
//   4. Where exactly does each String get dropped?
//
// The two commented-out lines would cause compiler errors.
// Write down WHY each one would fail before running.
//
// Run: cargo run --bin predict

fn takes_ownership(s: String) {
    println!("got: {}", s);
}

fn makes_copy(n: i32) {
    println!("copy of n: {}", n);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // <- why would this fail?

    let n = 42;
    makes_copy(n);
    println!("n is still: {}", n);

    let s2 = String::from("world");
    let s3 = s2;
    // println!("{}", s2); // <- why would this fail?
    println!("s3: {}", s3);

    let s4 = String::from("rust");
    let s5 = s4.clone();
    println!("s4: {}, s5: {}", s4, s5);
}
