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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    todo!()
}

fn main() {
    println!("{:?}", opposite(Direction::North));
    println!("{:?}", opposite(Direction::East));
    println!("{}", is_horizontal(&Direction::East));
    println!("{}", is_horizontal(&Direction::North));
    println!("{}", describe(&Direction::South));
    println!("{}", value_in_cents(Coin::Quarter));
    println!("{}", value_in_cents(Coin::Penny));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
