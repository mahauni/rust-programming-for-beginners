// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
    name: String
}

fn purchase(c: Customer) -> Result<String, String> {
    if c.age > 21 {
        Ok("You can purchase the item with success".to_owned())
    } else {
        Err("You can't buy the thing because you don't have the necessary age".to_owned())
    }
}

fn main() {
    let p1 = Customer {
        age: 30,
        name: "Lucas".to_owned()
    };

    let p2 = Customer {
        age: 17,
        name: "Davi".to_owned()
    };

    println!("Your purchase {}", p1.name);
    match purchase(p1) {
        Ok(m) => println!("{}", m),
        Err(m) => println!("{}", m)
    }

    println!("Your purchase {}", p2.name);
    match purchase(p2) {
        Ok(m) => println!("{}", m),
        Err(m) => println!("{}", m)
    }
}
