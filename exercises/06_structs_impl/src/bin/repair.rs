// TOPIC 06 — Structs and impl
// EXERCISE TYPE: Repair
// =============================================
// Fix the struct and impl errors.
//
// Run: cargo run --bin repair

struct Circle {
    radius: f64,
}

impl Circle {
    // ERROR 1: new() should be an associated function (no self), but it has &self
    fn new(&self, radius: f64) -> Circle {
        Circle { radius }
    }

    // ERROR 2: area() returns the wrong formula (missing the square)
    fn area(self) -> f64 {
        std::f64::consts::PI * self.radius
    }

    // ERROR 3: describe() tries to use self.name but the struct has no name field
    fn describe(&self) -> String {
        format!("Circle named {} with radius {}", self.name, self.radius)
    }
}

fn main() {
    let c = Circle::new(5.0);
    // ERROR 4: after calling area() above, c has been consumed — fix area() signature
    println!("area: {}", c.area());
    println!("describe: {}", c.describe());
}
