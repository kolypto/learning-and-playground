use std::error::Error;


// The `Box<dyn Error>` is a "trait object". For now, you can read it to mean "any kind of error"
fn main() -> Result<(), Box<dyn Error>> {
    // Rust has two types:
    // `Result<T, E>` for recoverable errors,
    // `panic!()` macro for unrecoverable errors





    // === Result<T, E> === //
    // enum Result<T,E> { Ok(T), Err(E) }

    // Example: open a file, handle result
    use std::fs::File;
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    // Matching on different errors
    use std::io::ErrorKind;
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // There's a lot of `match`.
    // In Chapter 13, you'll learn about closures. They can be more concise: e.g. `unwrap_or_else()`
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // The `Result<T,E>` type has many helper methods to make handling more concise.
    // `unwrap()`: it will return the Ok(_) value, or panic.
    let greeting_file = File::open("hello.txt").unwrap();
    // `expect()`: it will return the Ok(_) value, or panic with our message
    let greeting_file = File::open("hello.txt").expect("hello.txt is missing");

    // Propagating errors
    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        // Open file
        let mut username_file = match File::open("hello.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),  // return Result<> with this error
        };

        // Read into string
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // Propagating errors: use the `?;` shortcut
    // `?` works like this:
    // * if the value is `Ok`, it's returned from this expression
    // * if the value is `Err`, it's returned from the whole function
    // The `?` operator can only be used in functions with a compatible return type.
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Simplify further: chaining, use `?`
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // But in the end, let's use the standard library :)
    use std::fs;
    let file_contents = fs::read_to_string("hello.txt");





    // panic!(): unwinds the stack and aborts.
    // OUTPUT: thread 'main' panicked at 'crash and burn', src/main.rs:8:5
    panic!("crash and burn");

    // panic by accessing an index beyond the range.
    // OUTPUT: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:12:5
    let v = vec![1, 2, 3];
    v[99];



    // Return OK from main()
    // main() can return any type that implements `std::process::Termination` trait: converts to an ExitCode.
    Ok(())  // unit
}
