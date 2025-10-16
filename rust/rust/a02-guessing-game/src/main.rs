/* Guessing game:
 * The program will generate a random integer 0..100
 * and tell the user whether your guess is too low or high
 */


// Default built-ins are called "prelude"
// You don't have to import them:
// https://doc.rust-lang.org/std/prelude/index.html


// Bring into scope: the Standard Library's IO for stdin/stdout
use std::io;

// Random numbers generator
// Added with: $ cargo add rand
// The `Rng` trait defines methods that random number generators implement
use rand::Rng;

// `Ordering` is an enum, with variants: Less, Greater, Equal
use std::cmp::Ordering;


fn main() {
    // Welcome message
    println!("Guess the number!");

    // Generate a random number
    // This is an immutable value: there's no `let mut`, just `let`
    //
    // `rng()` is a random number generator: local to the current thread (thread_rng feature), seeded by the OS.
    // this method is brought into scope with the `use rand::Rng` statement.
    // `gen_range()` takes a range expression and generates a random number.
    //
    // Range expression: `start..=end`. It's inclusive on the lower and upper bounds.
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}"); // Cheat line. You're not supposed to see this.

    // Loop. Allows the user to guess multiple times.
    loop {
        // Prepare a var to read into.
        //
        // Create a variable to store user input
        // Variables are immutable by default, we add `mut` to make the value mutable.
        let mut guess = String::new();  // `String`: a growable UTF-8 encoded text

        // Read from stdin.
        //
        // `stdin()` returns an instance of std::io::Stdin. It handles standard input.
        //
        // `.read_line()` reads the input into `&mut guess`: a reference to `guess` (not a copy).
        // References are also immutable by default, hence we use `&mut`, not just `&guess`
        //
        // `.expect()` handles failures: it will crash and display the error.
        // It is a method of `Result`: an enum: a type that can be in one of multiple possible states.
        // Result can be: `Ok`, `Err`. The `Err` variant contains information about how the operation failed.
        println!("Input your guess:");
        io::stdin()
            .read_line(&mut guess)  // read into mutable reference
            .expect("Failed to read line"); // crash on failure
        println!("You guessed: {guess}");   // String with a placeholder.
        println!("You guessed: {}", guess); // Alternatively: provide vals as arguments

        // Convert `guess` string to u32.
        // Rust allows to "shadow" the previous variable.
        let guess: u32 = match guess
            .trim()  // remove whitespace
            .parse() // convert to another type: the type of the variable
            // Crash on error; but we would rather add error handling
            // .expect("Please enter a number!");
            { // This is a "match expression":
                // When ok, use the value. It will be returned.
                Ok(num) => num,
                // When failed, continue the loop.
                // The Err(_) is a catch-all pattern: matches all `Err` values
                Err(_) => continue,
            };

        // Compare the guess to the secret number
        match guess.
            cmp(&secret_number)  // `.cmp()` compares the numbers and returns a `std::cmp::Ordering`
            { // `match` expression has three "arms": patterns to match against. They are checked in turn.
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
