// Read ../../README.md before starting.

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

    let msg = Message::Write(String::from("world"));
    if let Message::Write(text) = msg {
        println!("if let matched: {}", text);
    }
}
