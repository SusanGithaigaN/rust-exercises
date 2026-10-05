// TOPIC 14 — Writing Tests
// EXERCISE 03: Test a struct with state
// =============================================
// A Counter struct is implemented below.
// YOUR TASK: write tests that verify its behavior across multiple method calls.
//
// Testing stateful structs means testing:
//   - Initial state after new()
//   - State changes after each method call
//   - Sequences of calls (increment, increment, reset, increment)
//   - Boundary behavior (what happens when the counter is full?)
//
// Example pattern for testing sequences:
//   let mut c = Counter::new(3);
//   assert_eq!(c.value(), 0);
//   assert!(c.increment());   // returns true, not at max
//   assert!(c.increment());
//   assert!(c.increment());   // now at max
//   assert!(!c.increment());  // returns false, already at max
//   assert_eq!(c.value(), 3);
//
// Run: cargo test --bin exercise_03

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

// TODO: write your tests here
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // your tests go here
// }

fn main() {
    let mut c = Counter::new(5);
    c.increment();
    c.increment();
    println!("value: {}, full: {}", c.value(), c.is_full());
    println!("Run 'cargo test --bin exercise_03' to run your tests.");
}
