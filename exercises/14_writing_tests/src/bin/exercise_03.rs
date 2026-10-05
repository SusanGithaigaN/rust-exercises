// Read ../../README.md before starting.

#[derive(Debug)]
struct Counter {
    value: i32,
    max: i32,
}

impl Counter {
    fn new(max: i32) -> Counter {
        Counter { value: 0, max }
    }

    fn increment(&mut self) -> bool {
        if self.value >= self.max {
            return false;
        }
        self.value += 1;
        true
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn is_full(&self) -> bool {
        self.value >= self.max
    }
}

// Write your tests here.

fn main() {
    let mut c = Counter::new(5);
    c.increment();
    c.increment();
    println!("value: {}, full: {}", c.value(), c.is_full());
    println!("Run 'cargo test --bin exercise_03' to run your tests.");
}
