// TOPIC 13 — Modules and Visibility
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. Can main access Outer::private_field directly?
//   2. Can the inner module access Outer's private field?
//   3. What does super:: mean?
//   4. What is the difference between pub and pub(crate)?
//
// Some lines are commented out intentionally.
// Predict whether each commented line would compile, and WHY.
//
// Run: cargo run --bin predict

mod outer {
    pub struct Config {
        pub name: String,
        secret_key: String,
    }

    impl Config {
        pub fn new(name: &str, key: &str) -> Config {
            Config {
                name: name.to_string(),
                secret_key: key.to_string(),
            }
        }

        pub fn is_valid(&self) -> bool {
            !self.secret_key.is_empty()
        }
    }

    pub(crate) fn describe(c: &Config) -> String {
        format!("Config({}, key={})", c.name, c.secret_key)
    }

    mod inner {
        use super::Config;

        pub fn validate(c: &Config) -> bool {
            !c.secret_key.is_empty() // can inner see private fields?
        }
    }
}

fn main() {
    let c = outer::Config::new("app", "abc123");
    println!("name: {}", c.name);
    // println!("key: {}", c.secret_key); // compiles?
    println!("valid: {}", c.is_valid());
    println!("{}", outer::describe(&c));
}
