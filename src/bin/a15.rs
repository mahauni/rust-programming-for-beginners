// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32)
}

fn main() {
    let v: Vec<Tickets> = vec![
        Tickets::Backstage(100.0, String::from("Lucas")),
        Tickets::Vip(200.0, String::from("Davi")),
        Tickets::Standard(50.0)
    ];

    for i in v {
        match i {
            Tickets::Vip(price, name) => println!("Vip ticket price: {}, name: {}", price, name),
            Tickets::Backstage(price, name) =>println!("Backstage ticket price: {}, name: {}", price, name),
            Tickets::Standard(price) => println!("Standart ticket price: {}", price)
        }
    }
}
