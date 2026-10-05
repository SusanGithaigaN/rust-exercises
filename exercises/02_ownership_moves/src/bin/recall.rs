// TOPIC 02 — Ownership: Moves and Scope
// EXERCISE TYPE: Recall
// =============================================
// Write two functions that work with owned Strings, then use them in main.
//
// 1. fn append_exclamation(s: String) -> String
//    Takes ownership of s, appends "!" to it, returns the new String.
//    Hint: use push_str or format!
//
// 2. fn shout(s: String) -> String
//    Takes ownership, returns the string in ALL CAPS.
//    Hint: s.to_uppercase()
//
// In main:
//   - Create a String "hello"
//   - Pass it to append_exclamation, get it back, print result
//   - Pass the result to shout, print the final value
//   - Show that you cannot use the original "hello" binding after the first move
//     (leave a commented-out line to make this visible)
//
// Expected output:
//   after append: hello!
//   after shout: HELLO!
//
// Run: cargo run --bin recall

fn main() {
    // TODO: implement the two functions above, then use them here
}
