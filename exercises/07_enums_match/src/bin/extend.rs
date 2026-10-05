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

fn duration_seconds(light: &TrafficLight) -> u32 {
    todo!()
}

fn next_light(light: TrafficLight) -> TrafficLight {
    todo!()
}

fn main() {
    println!("{:?}", opposite(Direction::North));
    println!("{}", is_horizontal(&Direction::East));
    println!("{:?}", turn_right(Direction::North));
    println!("{:?}", turn_right(Direction::West));
    println!("{}", duration_seconds(&TrafficLight::Red));
    println!("{:?}", next_light(TrafficLight::Red));
    println!("{:?}", next_light(TrafficLight::Green));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
