// TOPIC 04 — Borrowing: Mutable References
// EXERCISE TYPE: Extend
// =============================================
// The three recall functions are provided. Add one more.
//
// ADD:
// fn capitalize_first(s: &mut String)
//   Capitalizes the first letter of s in place.
//   If s is empty, do nothing.
//
//   Approach:
//     1. Check if empty
//     2. Get the first char: s.chars().next()
//     3. Build: format!("{}{}", first.to_uppercase(), &s[first.len_utf8()..])
//     4. Assign back to *s (or s.clear() + s.push_str(...))
//
// Expected output:
//   capitalize "hello": Hello
//   capitalize "rust is great": Rust is great
//   capitalize "": (empty, no change)
//
// Run: cargo run --bin extend

fn append_greeting(s: &mut String) {
    s.push_str(", hello!");
}

fn double_all(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn clamp_to_zero(n: &mut i32) {
    if *n < 0 {
        *n = 0;
    }
}

// TODO: add capitalize_first here

fn main() {
    let mut s = String::from("world");
    append_greeting(&mut s);
    println!("greeting: {}", s);

    let mut v = vec![1, 3, 5, 7];
    double_all(&mut v);
    println!("doubled: {:?}", v);

    // TODO: test capitalize_first here
}
