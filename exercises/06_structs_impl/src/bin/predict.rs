// TOPIC 06 — Structs and impl
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. What does #[derive(Debug)] do? What would happen without it?
//   2. What is the difference between new() and the other methods?
//      (Hint: one has &self, one does not — why?)
//   3. What does scale() return — a new value or the same one modified?
//   4. After calling scale(), can you still use rect?
//
// Run: cargo run --bin predict

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }
}

fn main() {
    let p = Point::new(3.0, 4.0);
    println!("{:?}", p);
    println!("distance: {}", p.distance_from_origin());

    let p2 = p.translate(1.0, 1.0);
    println!("translated: {:?}", p2);
    println!("original after translate: {:?}", p); // still valid?

    let p3 = Point { x: 0.0, y: 0.0 };
    println!("origin distance: {}", p3.distance_from_origin());
}
