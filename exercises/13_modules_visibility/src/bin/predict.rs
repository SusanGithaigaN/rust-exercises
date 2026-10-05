// Read ../../README.md before starting.

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
            !c.secret_key.is_empty()
        }
    }
}

fn main() {
    let c = outer::Config::new("app", "abc123");
    println!("name: {}", c.name);
    // println!("key: {}", c.secret_key);
    println!("valid: {}", c.is_valid());
    println!("{}", outer::describe(&c));
}
