// TOPIC 07 — Enums and match
// EXERCISE TYPE: Repair
// =============================================
// Fix the enum and match errors.
//
// Run: cargo run --bin repair

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn describe_season(s: Season) -> &'static str {
    // ERROR 1: match is not exhaustive — Winter is missing
    match s {
        Season::Spring => "warm and rainy",
        Season::Summer => "hot",
        Season::Autumn => "cool and windy",
    }
}

fn area(shape: Shape) -> f64 {
    // ERROR 2: wrong pattern syntax for tuple variants
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn main() {
    println!("{}", describe_season(Season::Winter));

    // ERROR 3: using enum variant without the enum name prefix
    let shape = Circle(3.0);
    println!("area: {}", area(shape));
}
