use std::{
    env,
    fmt::{Debug, Display},
};

// #[derive(Debug)]
struct Person {
    name: String,
    age: i64,
}

impl Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Debuging Person")
            .field("This is field!", &"This is value!")
            .field("name", &self.name)
            .field("old", &self.age)
            .finish()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "This is display! \nname => {}\nage => {}",
            self.name, self.age
        )
    }
}

#[test]
fn test_debug_derive() {
    let p = Person {
        name: "Seung Woo".to_string(),
        age: 19,
    };
    println!("{:?}", p);
    println!("{:}", p);
    println!("{}", p);
}
