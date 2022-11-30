// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(Debug)]
enum Power {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

fn get_input() -> io::Result<Power> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    match buf.trim().to_owned().as_str() {
        "off" => Ok(Power::Off),
        "sleep" => Ok(Power::Sleep),
        "reboot" => Ok(Power::Reboot),
        "shutdown" => Ok(Power::Shutdown),
        "hibernate" => Ok(Power::Hibernate),
        _ => {
            println!("Power not available, type a existent power");
            get_input()
        }
    }
}

fn print_power(s: Power) {
    match s {
        Power::Off => println!("Computer off"),
        Power::Hibernate => println!("Computer hibernating"),
        Power::Sleep => println!("Computer sleeping"),
        Power::Reboot => println!("Computer rebooting"),
        Power::Shutdown => println!("Computer shutting down")
    }
}

fn main() {
    println!("Type the power computer: ");
    let p = get_input();
    match p {
        Ok(s) => print_power(s),
        Err(err) => println!("Error: {}", err)
    }
}
