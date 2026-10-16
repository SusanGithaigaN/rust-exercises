// Read ../../README.md before starting.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Return the direction directly opposite: North↔South, East↔West.
fn opposite(d: Direction) -> Direction {
    todo!()
}

// True for East and West; false for North and South.
fn is_horizontal(d: &Direction) -> bool {
    todo!()
}

// Return "heading north", "heading south", "heading east", or "heading west".
fn describe(d: &Direction) -> &'static str {
    todo!()
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Return the value in cents: Penny=1, Nickel=5, Dime=10, Quarter=25.
fn value_in_cents(coin: Coin) -> u32 {
    todo!()
}

fn main() {
    println!("{:?}", opposite(Direction::North));     // South
    println!("{:?}", opposite(Direction::East));      // West
    println!("{}", is_horizontal(&Direction::East));  // true
    println!("{}", is_horizontal(&Direction::North)); // false
    println!("{}", describe(&Direction::South));      // heading south
    println!("{}", value_in_cents(Coin::Quarter));    // 25
    println!("{}", value_in_cents(Coin::Penny));      // 1
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
