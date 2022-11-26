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

struct Person {
    name: String,
    age: i32,
    color: String
}

fn print_person(v: Vec<Person>) {
    for i in v {
        match i.age {
            1 ..= 10 => {
                println!("name: {}", i.name);
                println!("color: {}", i.color)},
            _ => ()
        }
    }
}

fn main() {
    let p1 = Person {
        name: String::from("Lucas"),
        age: 9,
        color: String::from("Black")
    };
    let p2 = Person {
        name: String::from("Davi"),
        age: 14,
        color: String::from("Yellow")
    };
    let p3 = Person {
        name: String::from("Enrico"),
        age: 10,
        color: String::from("Purple")
    };

    let v = vec![p1, p2, p3];

    print_person(v)
}
