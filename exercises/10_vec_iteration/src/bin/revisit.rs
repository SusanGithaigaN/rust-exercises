// TOPIC 10 — Vec and Iteration
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// Given: let words = vec!["apple", "banana", "cherry", "avocado", "blueberry", "fig"];
//
// Using iterator methods only (no manual index loops):
//
// 1. Count words longer than 5 characters
// 2. Collect all words that start with 'a' or 'b'
// 3. Find the longest word (use .max_by_key(|w| w.len()))
// 4. Build one String by joining all words with ", "
//    Hint: words.join(", ")
// 5. Collect the first letter of each word into a Vec<char>
// 6. Count how many words contain the letter 'e'
//
// Expected output:
//   longer than 5: 3
//   starts with a or b: ["apple", "banana", "avocado", "blueberry"]
//   longest: "blueberry"
//   joined: "apple, banana, cherry, avocado, blueberry, fig"
//   first letters: ['a', 'b', 'c', 'a', 'b', 'f']
//   contains 'e': 4
//
// Run: cargo run --bin revisit

fn main() {
    let words = vec!["apple", "banana", "cherry", "avocado", "blueberry", "fig"];
    // TODO: compute and print each result above using iterator methods
}
