// TOPIC 03 — Borrowing: Shared References
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Write a text analysis function.
//
// fn analyze(text: &str) -> (usize, usize, usize, bool)
//   Returns a tuple of:
//     (word count, char count, sentence count, has_question)
//   - word count: words separated by whitespace
//   - char count: total characters
//   - sentence count: count of '.', '!', '?'
//   - has_question: true if text contains '?'
//
// In main:
//   - Call analyze with at least two different strings
//   - After each call, prove the original string is still usable
//
// Expected output (approximate):
//   "Hello world. How are you?" ->
//     words: 5, chars: 25, sentences: 2, has_question: true
//   original still here: Hello world. How are you?
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement analyze above main, then call here
}
