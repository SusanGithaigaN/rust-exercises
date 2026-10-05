// TOPIC 02 — Ownership: Moves and Scope
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Model a simple log entry system.
//
// fn create_entry(timestamp: String, message: String) -> String
//   Takes ownership of both strings, returns a combined entry:
//   "[{timestamp}] {message}"
//
// fn prefix_entry(prefix: String, entry: String) -> String
//   Takes ownership of both, returns: "{prefix}: {entry}"
//
// In main:
//   1. Create a timestamp string and a message string
//   2. Create an entry from them (they are consumed)
//   3. Create a prefix "INFO"
//   4. Create a final string from prefix + entry (both consumed)
//   5. Print the final result
//   6. Add a commented line showing that timestamp/message/entry/prefix are gone
//
// Expected output:
//   INFO: [2024-01-01] Server started
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement create_entry and prefix_entry above main, then call here
}
