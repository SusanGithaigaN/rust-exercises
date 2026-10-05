// TOPIC 07 — Enums and match
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. What does each call to process() print?
//   2. In the match inside process(), what does the {x, y} syntax do?
//   3. What happens to msg after it is passed to process()?
//   4. Why does the match need to handle every variant?
//
// Run: cargo run --bin predict

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({}, {})", x, y),
        Message::Write(text) => println!("write: {}", text),
        Message::Color(r, g, b) => println!("color: #{:02x}{:02x}{:02x}", r, g, b),
    }
}

fn main() {
    process(Message::Move { x: 3, y: 7 });
    process(Message::Color(255, 128, 0));
    process(Message::Write(String::from("hello")));
    process(Message::Quit);

    // if let example — predict the output:
    let msg = Message::Write(String::from("world"));
    if let Message::Write(text) = msg {
        println!("if let matched: {}", text);
    }

    // What does this print?
    let coin_value = |c: u32| if c == 1 { "penny" } else { "other" };
    println!("{}", coin_value(1));
    println!("{}", coin_value(25));
}
