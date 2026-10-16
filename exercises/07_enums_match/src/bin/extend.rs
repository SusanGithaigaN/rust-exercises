// Read ../../README.md before starting.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn opposite(d: Direction) -> Direction {
    todo!()
}
fn is_horizontal(d: &Direction) -> bool {
    todo!()
}
fn describe(d: &Direction) -> &'static str {
    todo!()
}

// Turning right: North→East, East→South, South→West, West→North.
fn turn_right(d: Direction) -> Direction {
    todo!()
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
    todo!()
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Red=30, Yellow=5, Green=25 (seconds).
fn duration_seconds(light: &TrafficLight) -> u32 {
    todo!()
}

// Sequence: Red→Green→Yellow→Red.
fn next_light(light: TrafficLight) -> TrafficLight {
    todo!()
}

fn main() {
    println!("{:?}", opposite(Direction::North));    // South
    println!("{}", is_horizontal(&Direction::East)); // true
    println!("{:?}", turn_right(Direction::North));  // East
    println!("{:?}", turn_right(Direction::West));   // North
    println!("{}", duration_seconds(&TrafficLight::Red));  // 30
    println!("{:?}", next_light(TrafficLight::Red));       // Green
    println!("{:?}", next_light(TrafficLight::Green));     // Yellow
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
