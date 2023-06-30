use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    is_employed: bool,
}


fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
        is_employed: true,
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_string);
    
    // parse json_string to Persion
    let parsed_person: Person = serde_json::from_str(&json_string).unwrap();
    println!("Parsed Person name : {}", parsed_person.name);
    println!("Parsed Person age: {}", parsed_person.age);
}
