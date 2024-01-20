#![allow(dead_code)]

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Box<Self> {
        Box::new(Self {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let people = vec![
        Person::new("Alice", 30),
        Person::new("Bob", 25),
        Person::new("Charlie", 40),
    ];

    for person in people.iter() {
        println!("{} is {} years old", person.name, person.age);
    }
}