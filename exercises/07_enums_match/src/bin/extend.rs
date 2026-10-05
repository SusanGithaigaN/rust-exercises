// TOPIC 07 — Enums and match
// EXERCISE TYPE: Extend
// =============================================
// The Direction enum from recall is provided. Add a new enum and one more function.
//
// ADD to Direction:
// fn turn_right(d: Direction) -> Direction
//   North->East, East->South, South->West, West->North
//
// ADD a new enum and its function:
// enum TrafficLight { Red, Yellow, Green }
// fn duration_seconds(light: &TrafficLight) -> u32
//   Red=30, Yellow=5, Green=25
// fn next_light(light: TrafficLight) -> TrafficLight
//   Red->Green, Green->Yellow, Yellow->Red
//
// Expected output:
//   turn right from North: East
//   turn right from West: North
//   Red lasts 30 seconds
//   next after Red: Green
//   next after Green: Yellow
//
// Run: cargo run --bin extend

#[derive(Debug)]
enum Direction { North, South, East, West }

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn is_horizontal(d: &Direction) -> bool {
    matches!(d, Direction::East | Direction::West)
}

fn describe(d: &Direction) -> &'static str {
    match d {
        Direction::North => "heading north",
        Direction::South => "heading south",
        Direction::East => "heading east",
        Direction::West => "heading west",
    }
}

// TODO: add turn_right here
// TODO: add TrafficLight enum, duration_seconds, and next_light here

fn main() {
    println!("opposite of North: {:?}", opposite(Direction::North));
    println!("East is horizontal: {}", is_horizontal(&Direction::East));
    println!("describing South: {}", describe(&Direction::South));
    // TODO: test turn_right and TrafficLight here
}
