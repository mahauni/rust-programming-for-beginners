// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    pub fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("Purple is not available".to_owned()),
            other => Ok(Self(other))
        }
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoes (color: Shoes) {
    println!("shoes color: {:?}", color)
}

fn print_pants (color: Pants) { 
    println!("pants color: {:?}", color)
}

fn print_shirt (color: Shirt) {
    println!("shirt color: {:?}", color)
}

fn main() {
    let shirt_color = Shirt::new(Color::Gray);
    let pants_color = Pants::new(Color::Blue);
    let shoes_color = Shoes::new(Color::White);

    print_pants(pants_color);
    print_shirt(shirt_color);

    match shoes_color {
        Ok(shoes) => print_shoes(shoes),
        Err(e) => println!("{}", e)
    };
}
