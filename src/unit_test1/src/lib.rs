// cargo new unit_test1 --lib
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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
    // run by: cargo test
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello Carol!`'
        assert!(
            result.contains("Carolx"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
