// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Bill {
    amount: i32,
    name: String
}

struct Bills {
    inner: HashMap<String, Bill>,
}

fn get_input() -> Option<String> {
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Please enter your data again")
    };

    let input = buf.trim().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, b: Bill) {
        self.inner.insert(b.name.clone(), b);
    }
    
    fn print(&mut self) {
        for i in self.inner.values() {
            println!("---------");
            println!("Name: {}", i.name);
            println!("Value: {}", i.amount);
            println!("---------")
        }
    }

    fn remove(&mut self, name: String) {
        self.inner.remove(&name);
    }

    fn edit(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }
}

fn add_bill(v: &mut Bills) {
    println!("Type the name of the bill:");
    let mut n = get_input();
    println!("Type the amount of the bill: ");
    let mut a = get_input();

    let name: String;
    let amount: i32;

    loop {
        match n {
            Some(s) => { name = s; break; },
            None => { println!("Type a valid amount"); n = get_input(); }
        }    
    }

    loop {
        match a {
            Some(n) => { amount = n.parse().unwrap(); break;},
            None => { println!("Type a valid amount"); a = get_input(); }
        }
    }
    
    let b = Bill {
        amount,
        name
    };

    Bills::add(v, b)
}

fn view_bill(v: &mut Bills) {
    Bills::print(v)
}

fn remove_bill(v: &mut Bills) {
    println!("Type the name of the item:");
    let mut rm = get_input();
    let name: String;

    loop {
        match rm {
            Some(s) => { name = s; break; },
            None => {
                println!("Type a valid name: ");
                rm = get_input();
            }
        }
    }

    Bills::remove(v, name)
}

fn update_bill(v: &mut Bills) {
    println!("Type the name of bill you want to update:");
    let mut n = get_input();
    println!("Type the amount of the bill: ");
    let mut a = get_input();

    let name: String;
    let amount: i32;

    loop {
        match n {
            Some(s) => { name = s; break; },
            None => { println!("Type a valid amount"); n = get_input(); }
        }    
    }

    loop {
        match a {
            Some(n) => {
                amount = n.parse().unwrap();
                break;
            },
            None => { println!("Type a valid amount"); a = get_input(); }
        }
    }
    
    let b = Bill {
        amount,
        name
    };

    Bills::edit(v, b) 
}

fn total_bill(v: &mut Bills) {
    let mut total = 0;
    for i in v.inner.values() {
        total += i.amount;
    }

    println!("You have a total of: {} bills", v.inner.len());
    println!("The total amount of the bills is {}", total)
}

fn main_menu() {
    fn menu() {
        println!("");
        println!("== Manaje your bills");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bills");
        println!("4. Update bill");
        println!("5. Bill total");
        println!("");
        println!("Enter selection:");
    }

    let mut bill = Bills::new();

    loop {
        menu();

        let input = get_input();

        match input {
            Some(s) => {
                match s.as_str() {
                    "1" => add_bill(&mut bill),
                    "2" => view_bill(&mut bill),
                    "3" => remove_bill(&mut bill),
                    "4" => update_bill(&mut bill),
                    "5" => total_bill(&mut bill),
                    _ => break,
                }
            },
            None => break
        }
    };

    println!("Thank you for using the app")
}

fn main() {
    main_menu();
}
