// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    is_big(101)
}

fn is_big(x: i32) {
    let lt_100 = x > 100;

    match lt_100 {
        true => println!("The number is more than 100"),
        false => println!("The number is less or iqual than 100")
    }
}
