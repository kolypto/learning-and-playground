// The function to test
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// The convention is to create a module "tests" in each file, and annotate the module with `#[cfg(test)]`:
// this annotation tells Rust to only compile this code when you run `cargo test`, not `cargo build`
#[cfg(test)]
mod tests {
    // Import all names from the root module
    use super::*;

    // A test is any function annotated with the "test" attribute
    #[test]
    fn it_works() {
        // NOTE: visibility: child modules can use the items from their ancestor modules.
        let result = add(2, 2);

        // `assert_eq!()` macro compares the two values. It will panic if the test fails.
        assert_eq!(result, 4);
        assert_ne!(result, 0);
        assert!(true);

        // With a custom error message
        assert!(result > 0, "There is a problem with {}", result);
    }

    // This test is expected to panic
    #[test]
    #[should_panic(expected="msg")] // this test is expected to panic, with a specific messages
    #[ignore]  // ignore this test unless specifically requested by name, with `$ cargo test -- --ignored`
    fn always_fails(){
        panic!("msg");
    }


    // A test that uses `Result<T, E>`
    // This test is passed when Ok() is returned.
    // Why? Because such a test enables you to use the `?` operator and fails immediately!
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 is not 4 in this galaxy"))
        }
    }
}
