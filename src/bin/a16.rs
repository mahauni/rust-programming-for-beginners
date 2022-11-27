// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Lockers {
    name: String,
    assign: Option<i32>
}

fn print(l: Lockers) {
    match l.assign {
        Some(i) => println!("{} has a locker at {}", l.name, i),
        None => println!("{} doesnt has a locker", l.name)
    }
}

fn main() {
    let s: Vec<Lockers> = vec![
        Lockers {
            name: "Lucas".to_owned(),
            assign: Some(14)
        },
        Lockers {
            name: "Enrico".to_owned(),
            assign: Some(21)
        },
        Lockers {
            name: "Davi".to_owned(),
            assign: None
        },
    ];

    for i in s {
        print(i)
    }
}
