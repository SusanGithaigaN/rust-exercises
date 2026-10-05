// TOPIC 13 — Modules and Visibility
// EXERCISE TYPE: Repair
// =============================================
// Fix the visibility and module path errors.
//
// Run: cargo run --bin repair

mod animals {
    pub struct Dog {
        pub name: String,
        age: u32, // private
    }

    impl Dog {
        pub fn new(name: &str, age: u32) -> Dog {
            Dog { name: name.to_string(), age }
        }

        pub fn description(&self) -> String {
            format!("{} (age {})", self.name, self.age)
        }
    }

    mod tricks {
        // ERROR 1: tricks is a private module — its contents are invisible outside animals
        pub fn sit(dog: &super::Dog) -> String {
            format!("{} sits", dog.name)
        }
    }
}

fn main() {
    let dog = animals::Dog::new("Rex", 3);

    // ERROR 2: accessing a private field from outside the module
    println!("age: {}", dog.age);

    // ERROR 3: calling a function in a private submodule from outside
    println!("{}", animals::tricks::sit(&dog));

    // ERROR 4: wrong path — trying to use Dog without the module prefix
    let dog2 = Dog::new("Buddy", 2);
    println!("{}", dog2.description());
}
