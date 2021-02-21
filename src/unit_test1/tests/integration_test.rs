/**
 In Rust, integration tests are entirely external to your library.
 Their purpose is to test whether many parts of your library work together correctly.
 Units of code that work correctly on their own could have problems when integrated,
 so test coverage of the integrated code is important as well.
**/
use unit_test1;
mod common;

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, unit_test1::add_two(2));
}