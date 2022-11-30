// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp() {
        assert_eq!(clamp(3, 10, 20), 10, "Should be 10");
        assert_eq!(clamp(20, 5, 7), 7, "Should be 7");
        assert_eq!(clamp(70, 70, 70), 70, "Should be 70");
        assert_eq!(clamp(70, 7, 170), 70, "Should be 70");
    }

    #[test]
    fn check_div() {
        assert_eq!(div(1, 3), Some(1/3), "Result should be Some(1/3");
        assert_eq!(div(7, 3), Some(7/3), "Result should be Some(7/3");
        assert_eq!(div(5, 5), Some(5/5), "Result should be Some(5/5)");
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat("Hello", "World"), "HelloWorld", "Result should be HelloWorld");
        assert_eq!(concat("Code", "Suc"), "CodeSuc", "Result should be CodeSuc");
        assert_eq!(concat("My", "House"), "MyHouse", "Result should be MyHouse");
    }

}

