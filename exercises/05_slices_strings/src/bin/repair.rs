// TOPIC 05 — Slices and Strings
// EXERCISE TYPE: Repair
// =============================================
// Fix all the errors. Read the error messages carefully.
//
// Run: cargo run --bin repair

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[0..i],
        None => s,
    }
}

fn main() {
    // ERROR 1: trying to slice at a position that may not be on a char boundary
    // (The index 3 slices into the middle of a multi-byte character)
    let s = "héllo";
    let slice = &s[0..3]; // fix: use a valid char boundary
    println!("{}", slice);

    // ERROR 2: s is borrowed as a slice, then s is mutated while borrow is alive
    let mut owned = String::from("hello world");
    let word = first_word(&owned);
    owned.push_str("!"); // this invalidates the borrow above
    println!("{}", word);

    // ERROR 3: type mismatch — assigning a &str slice to a String variable
    let text = "some text";
    let portion: String = &text[0..4];
    println!("{}", portion);
}
