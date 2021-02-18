# The Anatomy of a Test Function

    #[cfg(test)]
      mod tests {
        // using the #[test] attribute
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
      
        #[test]
        fn another() {
            // Tests fail when something in the test function panics. 
            panic!("Make this test fail");
        }
    }
  
  Checking Results with the assert! Macro
  The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
  
  Testing Equality with the assert_eq! and assert_ne! Macros

## Checking Results with the assert! Macro
The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.

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

        // The can_hold method returns a Boolean
        assert!(larger.can_hold(&smaller));
    }

## Testing Equality with the assert_eq! and assert_ne! Macros

    #[test]
    fn it_adds_two() {
        assert_eq!(4, 2+2);
    }
    
    #[test]
    fn it_adds_two_ne() {
        assert_eq!(4, 2+3);
    }
    
## Adding Custom Failure Messages
You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. 

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    
  ## Checking for Panics with should_panic
  should_panic! This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.
  
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    // Testing that a condition will cause a panic! with a particular panic message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

## Using Result<T, E> in Tests
We can also write tests that use Result<T, E>

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
