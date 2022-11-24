// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    qtd: i32,
    id: i32
}

fn quantity(q: &Grocery) {
    println!("{}", q.qtd)
}

fn identif(i: &Grocery) {
    println!("{}", i.id)
}

fn main() {
    let gross = Grocery {
        qtd: 12,
        id: 40
    };

    quantity(&gross);
    identif(&gross);
}
