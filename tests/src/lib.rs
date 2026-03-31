#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Created a guess with a invalid value, {value}");
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_guess() {
        Guess::new(101);
    }

    #[test]
    fn greeting_contains_name() {
        let greeting = greeting("Nick");
        assert!(greeting.contains("Nick"),
            "Greeing did not contain name, value was {greeting}");
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    fn larger_holds_smaller() {
        let larger = Rectangle{height: 5, width: 6};
        let smaller = Rectangle{height: 3, width: 5};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle{height: 10, width: 11};
        let smaller = Rectangle{height: 3, width: 10};

        assert!(!smaller.can_hold(&larger));
    }
}
