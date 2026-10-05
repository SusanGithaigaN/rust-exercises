// Read ../../README.md before starting.

mod animals {
    pub struct Dog {
        pub name: String,
        age: u32,
    }

    impl Dog {
        pub fn new(name: &str, age: u32) -> Dog {
            Dog {
                name: name.to_string(),
                age,
            }
        }

        pub fn description(&self) -> String {
            format!("{} (age {})", self.name, self.age)
        }
    }

    mod tricks {
        pub fn sit(dog: &super::Dog) -> String {
            format!("{} sits", dog.name)
        }
    }
}

fn main() {
    let dog = animals::Dog::new("Rex", 3);

    // ERROR 1
    println!("age: {}", dog.age);

    // ERROR 2
    println!("{}", animals::tricks::sit(&dog));

    // ERROR 3
    let dog2 = Dog::new("Buddy", 2);
    println!("{}", dog2.description());
}
