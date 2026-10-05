// Read ../../README.md before starting.

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[0..i],
        None => s,
    }
}

fn main() {
    // ERROR 1 — invalid byte index for this string
    let s = "héllo";
    let slice = &s[0..2];
    println!("{}", slice);

    // ERROR 2
    let mut owned = String::from("hello world");
    let word = first_word(&owned);
    owned.push_str("!");
    println!("{}", word);

    // ERROR 3
    let text = "some text";
    let portion: String = &text[0..4];
    println!("{}", portion);
}
