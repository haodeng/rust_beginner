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

# Test Organization
The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. 
Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

## Unit Tests
The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected. You’ll put unit tests in the src directory in each file with the code that they’re testing. 
The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Testing Private Functions
        #[test]
        fn internal() {
            assert_eq!(4, internal_adder(2, 2));
        }
        
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
    
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test. This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included. 
You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

## Integration Tests
In Rust, integration tests are entirely external to your library. 
Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. 
To create integration tests, you first need a tests directory.

Filename: tests/integration_test.rs, We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 

    use adder;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, adder::add_two(2));
    }
    
To run all the tests in a particular integration test file, use the --test argument

    // This command runs only the tests in the tests/integration_test.rs file.
    cargo test --test integration_test
    
## Submodules in Integration Tests
Filename: tests/common/mod.rs.

    pub fn setup() {
        // setup code specific to your library's tests would go here
    }

Filename: tests/integration_test.rs

    use adder;

    mod common;

    #[test]
    fn it_adds_two() {
        common::setup();
        assert_eq!(4, adder::add_two(2));
    }
