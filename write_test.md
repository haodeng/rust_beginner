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

# Controlling How Tests Are Run

    // compiles your code in test mode and runs the resulting test binary
    cargo test 
    
    // displays the options you can use with cargo test
    cargo test --help
    
    // displays the options you can use after the separator --.
    cargo test -- --help 
    
## Running Tests in Parallel or Consecutively
When you run multiple tests, by default they run in parallel using threads. 
If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary.

    // 1, telling the program not to use any parallelism.
    cargo test -- --test-threads=1
    
## Showing Function Output
By default, if a test passes, Rust’s test library captures anything printed to standard output. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

    // If we want to see printed values for passing tests as well, 
    // we can tell Rust to also show the output of successful tests at the end with --show-output.
    cargo test -- --show-output

## Running a Subset of Tests by Name

    // Running Single Tests by name
    cargo test one_hundred
    
    // Filtering to Run Multiple Tests
    // specify part of a test name, and any test whose name matches that value will be run
    // ran all tests with add in the name and filtered out other tests
    cargo test add
    
## Ignoring Some Tests Unless Specifically Requested

    #[test]
    // using the ignore attribute to exclude them
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

If we want to run only the ignored tests, we can use

    cargo test -- --ignored
