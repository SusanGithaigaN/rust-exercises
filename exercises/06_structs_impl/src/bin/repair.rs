// Read ../../README.md before starting.

struct Circle {
    radius: f64,
}

impl Circle {
    // ERROR 1
    fn new(&self, radius: f64) -> Circle {
        Circle { radius }
    }

    // ERROR 2
    fn area(self) -> f64 {
        std::f64::consts::PI * self.radius
    }

    // ERROR 3
    fn describe(&self) -> String {
        format!("Circle named {} with radius {}", self.name, self.radius)
    }
}

fn main() {
    let c = Circle::new(5.0);
    // ERROR 4 — also caused by ERROR 2's signature
    println!("area: {}", c.area());
    println!("describe: {}", c.describe());
}
