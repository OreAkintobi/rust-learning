// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    name: String,
    fave_color: String,
    age: i32,
}

// * The name and colors should be printed using a function
fn print_item(item: &str) -> () {
    println!("{:?}", item);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let person_1 = Person {
        name: String::from("Stine"),
        age: 8,
        fave_color: String::from("SandalWood"),
    };

    let person_2 = Person {
        name: "Anna".to_owned(),
        age: 10,
        fave_color: String::from("Red"),
    };

    let person_3 = Person {
        name: String::from("Sophie"),
        age: 22,
        fave_color: String::from("Blue"),
    };

    let people = vec![person_1, person_2, person_3];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            // * The name and colors should be printed using a function
            print_item(&person.name);
            print_item(&person.fave_color);
        }
    }
}
