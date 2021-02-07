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
