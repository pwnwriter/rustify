struct Dog {
    name: String,
    age: u32,
    weight: u32,
    color: String,
}

pub fn structs() {
    let mut dog1 = Dog {
        name: String::from("wip"),
        age: 4,
        weight: 8,
        color: String::from("red"),
    };

    println!(
        "Name of the dog is {}, age is {} with weight of {} and {} color ",
        dog1.name, dog1.age, dog1.weight, dog1.color
    );

    // Changing the value

    dog1.name = String::from("cutee");
    println!("I changed the dog name to {}", dog1.name);
}
