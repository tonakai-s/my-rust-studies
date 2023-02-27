pub fn is_even(value: i32) -> Result<i32, String> {
    if value % 2 == 0 {
        Ok(value)
    } else {
        Err(format!("The number {} is odd.", value))
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess need to be greater than 1. Got {}", value);
        }
        if value > 100 {
            panic!("Guess need to be less than 100. Got {}", value);
        }

        Guess {
            value: value
        }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_includes_name(){
        let result = greeting("Lucas");
        assert!(
            result.contains("Lucas"),
            "Result need to have the name, but was {}", result
        );
    }

    #[test]
    fn exploration() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    // #[test]
    // fn another(){
    //     panic!("Make this code fail");
    // }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 35,
            height: 80,
        };

        let smaller = Rectangle {
            width: 20,
            height: 79,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 60,
            height: 90,
        };

        let smaller = Rectangle {
            width: 20,
            height: 30,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected="Guess need to be less than 100.")]
    fn panic_out_of_range(){
        Guess::new(200);
    }

    #[test]
    fn is_even_test() {
        let result = is_even(6);
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn is_odd_test() {
        let result = is_even(5);
        assert!(result.is_err());
    }
}
