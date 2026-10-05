// TOPIC 08 — Option<T>
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK A: fn parse_age(s: &str) -> Option<u32>
//   Returns None if:
//   - s is empty
//   - s doesn't parse to a number
//   - the number is > 150
//   Otherwise returns Some(n).
//   Hint: s.parse::<u32>().ok() returns Option<u32>
//
// TASK B: fn greet(name: Option<&str>) -> String
//   Returns "Hello, {name}!" if Some, or "Hello, stranger!" if None.
//
// TASK C: fn find_matching(v: &[String], prefix: &str) -> Option<&String>
//   Returns the first String in v that starts with prefix, or None.
//
// Expected output:
//   parse_age("25"): Some(25)
//   parse_age("abc"): None
//   parse_age("200"): None
//   greet(Some("Alice")): Hello, Alice!
//   greet(None): Hello, stranger!
//   find_matching prefix "ru": Some("rust")
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement parse_age, greet, and find_matching above main, then use here
}
