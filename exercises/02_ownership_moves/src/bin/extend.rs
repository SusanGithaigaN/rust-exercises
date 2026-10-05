// TOPIC 02 — Ownership: Moves and Scope
// EXERCISE TYPE: Extend
// =============================================
// The two functions from recall are provided. Now add a third that works
// with a Vec<i32> instead of a String.
//
// ADD:
// fn double_elements(v: Vec<i32>) -> Vec<i32>
//   Takes ownership of the Vec, doubles every element, returns a new Vec.
//   Hint: v.into_iter().map(|x| x * 2).collect()
//
// Also: write fn sum_and_drop(v: Vec<i32>) -> i32
//   Takes ownership, sums all elements, returns the sum.
//   The Vec is dropped at the end of this function.
//
// In main:
//   - Create a Vec, pass to double_elements, get it back, print it
//   - Create another Vec, pass to sum_and_drop, print the sum
//   - Show (with a commented line) that the second Vec is gone after sum_and_drop
//
// Expected output:
//   doubled: [2, 4, 6, 8, 10]
//   sum: 15
//
// Run: cargo run --bin extend

fn append_exclamation(s: String) -> String {
    format!("{}!", s)
}

fn shout(s: String) -> String {
    s.to_uppercase()
}

// TODO: add double_elements and sum_and_drop here

fn main() {
    let s = String::from("hello");
    let s = append_exclamation(s);
    println!("{}", shout(s));
    // TODO: add Vec exercises here
}
