# Go





# rust/a01-hello/src


# rust/a01-hello/src/main.rs

```rust
// Create a new app with:
// $ cargo new a01-hello

// Run me with:
// $ cargo run
// or
// $ rustc main.rs
// $ ./main

// To build it:
// For release, with optimizations:
// $ cargo build --release

// Dependency added with:
// $ cargo add ferris-says
use ferris_says::say; // use function `say()` from the crate
use std::io::{stdout, BufWriter};

fn main() {
    // Simple print.
    // This is a Rust macro: as indicated by the `!`
    println!("Hello, world!");
    
    // Print using Ferris-Says
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let message_length = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, message_length, &mut writer).unwrap();
}

```





# rust/a01-hello


# rust/a01-hello/Cargo.toml

```toml
[package]
name = "a01-hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
ferris-says = "0.3.1"

```





# rust/a02_guessing_game/src


# rust/a02_guessing_game/src/main.rs

```rust
/* The program will generate a random integer 0..100 and tell the user whether their guess is too low or high
 */


// Default built-ins are called "prelude"
// Import a library:
use std::io;

// Random numbers generator
// The `Rng` trait defines methods that random number generators implement
use rand::Rng;

// `Ordering` is an enum, with variants: Less, Greater, Equal
use std::cmp::Ordering;

fn main() {
    // Welcome message
    println!("Guess the number!");

    // Generate a random number
    // This is an immutable value: there's not `let mut`
    // `thread_rng()` is a random number generator: local to the current thread, seeded by the OS.
    // this method is brought into scope with the `use rand::Rng` statement.
    // `gen_range()` takes a range expression and generates a random number.
    // Range expression: `start..=end`. It's inclusive on the lower and upper bounds.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}"); // Cheat line
    
    // Loop. Allows the user to guess multiple times.
    loop {
        // Input from the user
        println!("Input your guess:");

        // Create a variable to store user input
        // Variables are immutable by default, we add `mut` to make the value mutable.
        // `String` is a growable UTF-8 encoded text
        // `::new()` is an "associated function" of a type: i.e. implemented on that type.
        let mut guess = String::new();

        // `stdin()` returns an instance of std::io::Stdin. It handles standard input.
        // `.read_line()` reads the input and appends it to `&mut guess`: a reference to `guess` (not a copy).
        // References are also immutable by default, hence we use `&mut`, not just `&guess`
        // `.expect()` handles failures: `.read_line()` returns a `Result`: an enum. It encodes error-handling information.
        // Result can be: `Ok`, `Err`. The `Err` variant contains information about how the operation failed.
        // `.expect()` will crash and display the error message.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Print the value.
        // `{guess}` is a placeholder: a variable name. It formats a variable value. 
        // Alternatively, use `{}` and provide the variable as an argument.
        println!("You guessed: {guess}");

        // Convert `guess` to u32.
        // Rust allows to "shadow" the previous variable. 
        // `.trim()` removes whitespace
        // `.parse()` converts to another type: the type of the variable.
        // Use `.parse().expect("...")` to crash on error; but we rather add error handling
        let guess: u32 = match guess
            .trim()
            .parse()
            // .expect("Please enter a number!");
            { // This is a "match expression"
                // When ok, use the value. It will be returned.
                Ok(num) => num,
                // When failed, continue the loop.
                // The Err(_) is a catch-all pattern: matches all `Err` values
                Err(_) => continue,
            };

        // Compare the guess to the secret number
        // `.cmp()` compares the numbers and returns a `std::cmp::Ordering`
        // `match` expression has three "arms": patterns to match against. They are checked in turn.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");

                // Quit after a correct guess
                break;
            },
        }
    }
}

```





# rust/a03_variables/src


# rust/a03_variables/src/main.rs

```rust
fn main() {
    // Variables are immutable by default: you can't change them. Rust compiler will guarantee that.
    #[allow(unused_variables)] // disable compiler warning
    let x = 5;  
    // x = 6;  // error[E0384]: cannot assign twice to immutable variable `x`

    // You make a variable mutable by using `mut`.
    // Variable shadowing is allowed. Variables are local to a scope { .. }
    #[allow(unused_assignments)]
    let mut x = 5;
    x = 6;
    println!("Value: x={x}");

    // Constants are always immutable.
    // They can only be set to a constant expression. It's evaluated at compile time.
    #[allow(dead_code)]
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // === Scalar types === //

    // Scalar types: int, float, bool, character
    // Integers: i8, i16, i32, i64, i128, isize (arch-dependent)
    // Unsigned: u8, u16, u32, u64, u128, usize (arch-dependent)
    // If an integer overflows: debug => panics, release => wraps around without failure
    let _x: i64 = 98_222;
    let _x: i64 = 0xFF;
    let _x: i64 = 0o755;
    let _x: i64 = 0b1111_0000;
    let _x: u8 = b'A';  // byte, `u8` only
    
    // Floats: f32, f64
    // Default is f64: same speed, but more precision
    let _x: f64 = 2.0;

    // Boolean
    let _t = true;
    let _t: bool = false;

    // Character: one UTF8 scalar value
    let _c = 'z';
    let _cat = '😻';



    // === Compound types. === //
    
    // Tuple. Groups together a variety of types into a compound type.
    // They cannot grow or shrink in size.
    let _tup: (i32, f64, u8) = (500, 3.14, 1);
    let (_x, _y, _z) = _tup; // destructure
    let _x = _tup.0; // get one value

    // "Unit": a tuple without any values. 
    // Represents an empty return type.
    // Expressions implicitly return the "unit" value if they don't return any other value.
    let _x = ();

    // Array. Elements must have the same type. Arrays are fixed-length.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    let _a = [1,2,3,4,5];
    let _months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // array type with size
    let _a = [0; 5]; // fill with 5 values `0`
    let _first = _a[0]; // access element
    // let _nonexistent = _a[5]; // will panic: "index out of bounds: the length is 5 but the index is 5"

    // Vector. Like array, but of a dynamic size.
}

```





# rust/a04_flow/src


# rust/a04_flow/src/main.rs

```rust
fn main() {
    println!("Hello, world!");

    // Function call
    let ret = another_function(42, 'h');
    println!("Return: {ret}");

    // `if` expression
    // Blocks of code associated with the condition are called "arms".
    // When too many arms, use `match`.
    let number = 3;
    if number < 5 {
        println!("number < 5");
    } else if number > 100 {
        println!("number > 100");
    } else {
        println!("number is somewhere between 5..100");
    }

    // Because `if` is an expression, we can use it in a `let`.
    // See expressions below
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Result: {number}");

    // `loop`: forever
    loop {
        println!("forever");
        
        break; // okay, not forever 
        #[allow(unreachable_code)]
        continue; // skip iteration   
    }

    // A `loop` can return a value: useful for retry operations
    let mut counter = 0;
    let result = loop {
        // Keep incrementing
        counter += 1;

        // Stop when reached 10 retries
        if counter == 10 {
            // This value will be returned out of the loop expression
            break counter * 2;
        }
    };
    println!("Counter result: {result}");

    // Loop labels: break out of many levels
    'out: loop {
        loop {
            break 'out;
        }
    }

    // Conditional loops
    let mut number = 3;
    while number >= 0 {
        number -= 1;
    }

    // Loop through a collection
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("a[]: {element}");
    }

    // Loop through a range
    for number in (1..4).rev() {
        println!("Countdown: {number}!");
    }
}

// Function definition
fn another_function(value: i32, unit: char) -> i32 {
    println!("Another function; value={value}{unit}.");

    // Statement: instructions that do not return a value
    // Expression: evaluate to a resultant value
    let _x = 6; // statement
    // let _x = (let _x = 6); // Fails: "note: variable declaration using `let` is a statement"

    // Expression: this block evaluates to 4
    let _x = {
        let x = 3;
        x + 1
    };

    // Return value: final expression returns implicitly
    // Expression: without ";"
    _x
}

```

