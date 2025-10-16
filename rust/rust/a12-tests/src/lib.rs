// This crate is a library, created with:
// $ cargo new <name> --lib

// The function to test
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// A test in Rust is a function that's annotated with the "test" attribute:
// #[test]
// When you run
// $ cargo test
// Rust builds a test runner binary that runs the annotated functions.

// When you create a library create, Rust generates a test function already in the same file:
// $ cargo new <name> --lib

// The convention is to create a module "tests" in each file, and annotate the module with `#[cfg(test)]`:
// this annotation tells Rust to only compile this code when you run `cargo test`, not `cargo build`.
//
// Alternatively, create a "tests/*.rs" directory and put your tests there.
// Every file in "tests/*.rs" is compiled as a separate crate.
#[cfg(test)] // Only build for `cargo test`, never for `cargo build`
mod tests {  // Tests module
    // Import all names from the root module.
    // Remember that in Rust submodules can have access to parent modules: because
    // submodules are "implementation detail" of the parent module.
    use super::*;

    // A test is any function annotated with the "test" attribute
    #[test]
    fn it_works() {
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
