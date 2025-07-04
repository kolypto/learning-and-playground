# Rust





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

# "Edition" is a major release of the language, released every couple of years.
# Edition changes only affect the way the compiler initially parses code:
# if you use a crate that wants an older edition, Rust will comply.
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





# rust/a05_ownership
## Ownership

Ownership: rules that govern how a program manages memory.
It's checked by the compiler. None of the features of ownership will slow down the program 
while it's running.

Memory is managed differently depending on whether it's in the stack or in the heap.

* Stack: FIFO, requires structures of a known size.
* Heap: allocate memory. It's slower because it requires more work to find a big enough space.

Accessing data on the heap is also slower: you have to follow a pointer to get there.

When a function is called, arguments and local variables get pushed onto the stack. 
When the function is over, those values get popped back off the stack. 

## Ownership rules

Rules:

* Each value in Rust has an *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

A declared variable is valid until the end of the current *scope*:

```rust
{
    let s = "hello";
    // `s` is still valid here
}

// `s` is no longer valid here: the scope is now over
```

Such values are of a known size and can be stored on the stack and popped off the stack
when their scope is over. String literals are immutable.

The `String` type, however, is mutable, and is allocated on the heap.
It can be mutated:

```rust
// Allocate memory
let mut s = String::from("hello");
s.push_str(", world!");
println!("{}", s);  //-> "hello, world!"
```

This string allocates a memory, and should return this memory to the allocator when we're done with it.
Rust automatically returns the memory once the variable that owns it goes out of scope: 
Rust calls a special function for us, `drop()`, and it's where the author of `String` can return the memory.

Here's what happens when multiple variables use a value:

on the stack:

```rust
// "bind" the value `5` to `x`
let x = 5;

// make a copy of the value in `x` and bind it to `y`
let y = x;
```

in the heap:

```rust
// "bind" the value to `s1`
// On the stack, it stores: 
//  `ptr` pointer to the heap, `len` length of the string, `capacity` the amount of allocated memory
// On the heap, it stores:
//  the actual string, "hello", located at `ptr` memory address
let s1 = String::from("hello");

// make a copy of the (`ptr`, `len`, `capacity`). The string data is not copied: it is referenced.
// It points to the same memory address, so it can't be freed when `s2` goes out of scope.
let s2 = s1;
```

To ensure memory safety, Rust considers `s1` as no longer valid, and doesn't need to free anything
when `s1` goes out of scope.
Here's what happens if you try to use `s1` after `s2` is created: Rust prevents you from using an invalidated reference:

```rust
println!("{}, world!", s1);
```

Error:

```
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:10:28
   |
7  |      let s1 = String::from("hello");
   |          -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
8  |     let s2 = s1;
   |              -- value moved here
9  |
10 |     println!("{}, world!", s1);
   |                            ^^ value borrowed here after move
```

Because Rust invalidates the first variable `s1` instead of making a *shallow copy*, it's known as a *move*. 
We would say that `s1` was *moved* into `s2`.

If we wanted to make a deep copy of the value including its heap data, we can use a common method `.clone()`.
Do this when the performance cost is acceptable:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```

### Stack-Only Data: Copy

When the data is entirely on the stack, copying is so cheap that Rust does it without asking:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied.
Such values are still valid after that assignment.
The `Copy` trait is only valid if the type, or any of its parts, has not implemented the `Drop` trait.

Simple scalar types do implement the `Copy` trait: integers, boolean, float, char, tuples. 

### Ownership and Functions

Passing a value to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // `s` comes into scope

    takes_ownership(s);             // `s`s value moves into the function...
                                    // ... and so is no longer valid here


    let x = 5;                      // `x` comes into scope

    makes_copy(x);                  // `x` would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use `x` afterward

} // Here, `x` goes out of scope, then `s`. 
  // But because `s`s value was moved, nothing special happens.


fn takes_ownership(some_string: String) { // `some_string` comes into scope
    println!("{}", some_string);
} // Here, `some_string` goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // `some_integer` comes into scope
    println!("{}", some_integer);
} // Here, `some_integer` goes out of scope. Nothing special happens.
```

### Return Values and Scope

Returning values can also transfer ownership:

```rust
fn main() {
    let s1 = gives_ownership();         // moves its return value into `s1`

    let s2 = String::from("hello");     // `s2` comes into scope

    let s3 = takes_and_gives_back(s2);  // `s2` is moved into `takes_and_gives_back`, 
                                        // its return value moves into `s3`
                                        
} // `s3` goes out of scope and is dropped. 
  // `s2` was moved, so nothing happens. 
  // `s1` goes out of scope and is dropped.



fn gives_ownership() -> String {             // will move its return value into the caller

    let some_string = String::from("yours"); // `some_string` comes into scope

    some_string                              // is returned and moves out to the caller
}

fn takes_and_gives_back(a_string: String) -> String { // `a_string` comes into scope

    a_string  // is returned and moves out to the caller
}
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. 
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership 
of the data has been moved to another variable.

But what if we want to let a function use a value but not take ownership?
We can return the same value just to have it moved back to where we've taken it.
Rust allows you to return multiple values using a tuple:

```rust
// It takes ownership of `s`, and then moves it back to the caller
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


// Here's how to use it
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
}
```

But this is too much ceremony.
Let's see how Rust lets you transfer references.









## References and Borrowing

Instead of returning the value to the caller, we can provide a reference to the `String` value.
A reference is like a pointer to a value owned by some other variable. 
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
// The `&String` is a reference
fn calculate_length(s: &String) -> usize {
    s.len()
} // the reference is dropped, but the value is not

// Here's how to use it
fn main() {
    let s1 = String::from("hello");

    // Create a reference that refers to the value of `s1` but does not own it.
    // This value will not be dropped when the reference stops being used.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
```

We call it *borrowing*: the action of creating a reference. You don't own it.

Such a reference is a double-pointer: `&String` points to a `String`, 
which in turn points to the heap where the string bytes are stored.

By the way, to dereference a pointer, you would do `*s`. 



### Mutable References

References are immutable by default: you cannot change the value you've borrowed.
To allow modifications, use a *mutable reference*:

```rust
// The function accepts a mutable reference
fn change(some_string: &mut String) {
    // mutate the value
    some_string.push_str(", world");
}

fn main() {
    // Make the String mutable
    let mut s = String::from("hello");

    // Create a mutable reference and pass it
    change(&mut s);
}
```

❗ Restriction: if you have a mutable reference to a value, you can have no other references to that value.
This prevents data races at compile time: you cannot modify something that would be a surprise to another function.
Simply put, Rust refuses to compile code with data races!

You can use curly braces to allow for multiple mutable references:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // `r1` goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 
That is, you can create a mutable reference where other references are not used any more:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
// variables `r1` and `r2` will not be used after this point

let r3 = &mut s; // no problem
```


### Dangling References

Rust will make sure there are no *dangling references*: a pointer to something that may be freed:

```rust
fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // here, `s` goes out of scope and is dropped. Its memory goes away. You cannot reference it anymore.
```

Error:

```
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
```

See Chapter 10: lifetimes.

The solution here is to return the `String` value directly:
the caller will take ownership of the returned value:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```







## Slices

*Slices* let you reference a section of an array.
A slice is a kind of reference, so it does not have ownership.

A *string slice* is a reference to a part of a `String`:

```rust
let s = String::from("hello world");

// Create slices using a range.
// A slice internally is a (pointer, length)
let hello = &s[..5]; // range: [0, 5)
let world = &s[6..]; // range: [6, 11)
let helloworld = &s[..]; // whole string
```

NOTE: String slice range indices must occur at valid UTF-8 character boundaries. 
If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

Let's write a function that takes a string, and returns the first word:
e.g. "hello world" returns just "hello".

```rust
fn first_word(s: &String) -> &str {
    // Convert `String` to an array of bytes
    let bytes = s.as_bytes();

    // Create an iterator: `.iter()` returns each element in a collection
    // `.enumerate()` returns tuples of (index, element)
    // We use patterns to destructure the tuple: `i` index, `&item` reference to the item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found, return the whole string
    &s[..]
}
```

The compiler will make sure that slices remain valid: i.e. that the original string remains unmodified:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // Modify the string 
    s.clear(); // error!
}
```

Error:

```
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. 
This is exactly what happens here: `.clear()` needs a mutable reference, and Rust disallows it.

One improvement of the function we've defined: 
because string literals are `&str`: that is, slices of a string, we can do this:

```rust
fn first_word(s: &str) -> &str {
```

now this function accepts both `String` and `str` slices.
This flexibility takes advantage of *deref coercions*: see Chapter 15.



## Other Slices

Slice of an array:

```rust
let a = [1, 2, 3, 4, 5];

// This slice has type &[i32]
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```




# rust/a06_structs/src


# rust/a06_structs/src/main.rs

```rust
#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // Define a struct
    #[derive(Debug)]  // attribute: for debug display
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Create an "instance" of that struct
    let mut user = User {
        active: true,
        username: String::from("john"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
    };

    // Change a field
    // NOTE: the entire instance must be mutable. You cannot mark individual fields as mutable.
    user.email = String::from("john@gmail.com");

    // Define a function that create an instance of the structure.
    // Here we use the "field init shorthand": to avoid repetition
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            // Shorthand for: `username: username`
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Create another struct: copy some fields, override others.
    // ❗ Note that the "struct update syntax" uses `=`: it moves the data. You cannot use `user.username` anymore.
    // ❗ But you can still use `user.email` because it's not moved.
    // ❗ And you can still use `user.active` and `user.sign_in_count` because they implement the `Copy` trait.
    let user2 = User {
        email: String::from("john@msn.com"),
        // copy values from corresponding fields
        ..user
    };

    // Here's how to dump values for debugging.
    // Format: "{:?}" uses the `Debug` trait to print the value in a way that is useful for developers.
    // The `User` struct implemented it using the attribute #[derive(Debug)]
    println!("{:?}", user2);  //-> User { active: true, username: "john", email: "john@msn.com", sign_in_count: 1 }
    println!("{:#?}", user2); //-> same, but with newlines. Easier to read.
    dbg!(user2); //-> [src/main.rs:51] user2 = User { ... }
    // The `dbg!()` macro can be used as an expression:
    user.email = dbg!(String::from("john@mail.ru")); //-> [src/main.rs:54] String::from("john@mail.ru") = "john@mail.ru"






    // A "tuple struct" without named fields. It's a tuple with a name.
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // Access
    let r = black.0;

    // Destructure a tuple struct
    let Color(r, g, b) = black;






    // A "unit-like" structure is a tuple without any fields.
    // Imagine: we can implement behavior for this type such that every instance is always equal 
    // to every instance of any other type!
    struct AlwaysEqual;
    let subject = AlwaysEqual;






    // Methods.
    // Unlike functions, methods are defined within the context of a struct, enum, or a trait object.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // Define a function in the context of `Rectangle`.
    // Everything within the `impl` block will be associated with the `Rectangle` type.
    // Multiple `impl` blocks can be defined: useful with generics and traits (see Chapter 10)
    impl Rectangle {
        // For methods, their first parameter is always `self`.
        // The `&self` is actually short for `self: &Self`, where `Self` is an alias for the type that the impl block is for.
        //
        // We need to use `&` to indicate that this method borrows the `Self` instance:
        // it's a reference because we don't want to take ownership.
        // Methods can take ownership of `self`, borrow `self` mutably or immutably.
        //
        // Having a method that takes ownership is rare: usually when the method transforms `self` into something else
        // and wants to prevent the caller from using the original instance.
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Define a method with the same name as the field.
        // Rust will know which one to choose: `.width` is a field, `.width()` is a method.
        //
        // Such "getters" can be useful if you make the field private, and the getter public.
        fn width(&self) -> bool {
            self.width > 0  // has width
        }

        // Define a method that uses another `Rectangle`:
        // check if it can fit completely withing `self`
        fn can_hold(&self, other: &Rectangle) -> bool {
            other.width <= self.width && other.height <= self.height
        }

        // Define a method that doesn't need `self`.
        // These are often used as constructors.
        // To call these, use `Rectangle::square()`. It's like a static method.
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    // Use the method
    // Rust automatically de/references the `self` when you use the `.` operator.
    let rect = Rectangle::square(32);
    println!("Area: {}", rect.area());
}

```





# rust/a07_enums_and_patterns/src


# rust/a07_enums_and_patterns/src/main.rs

```rust
#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // An Enum encodes meaning along with data. 
    // It says that a value is one of a possible set of values.
    enum IpAddressKind {
        V4,
        V6,
    }
    let kind = IpAddressKind::V4;

    // We can put data directly into each enum variant: it will have an associated value:
    enum IpAddress {
        V4(String),
        V6(String),
    }

    // Every enum variant is also a function that constructs an instance of the enum:
    let loopback = IpAddress::V4(String::from("127.0.0.1"));

    // Another advantage: enum variants can have different types associated with them:
    enum IpAddress2 {
        // We can use individual bytes.
        // The standard library uses structs.
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress2::V4(127, 0, 0, 1);
    let loopback = IpAddress2::V6(String::from("::1"));    

    // More variants with embedded types:
    enum Message {
        // No data
        Quit,
        // named fields, like a struct
        Move { x: i32, y: i32 },
        // a String
        Write(String),
        // three ints
        ChangeColor(i32, i32, i32),
    }

    // We could have defined 4 structs instead, but then it wouldn't be so easy 
    // to define a function to take any of these kinds of messages.

    // We can define methods on enums
    impl Message {
        fn call(&self){
            // ...
        }
    }
    Message::Write(String::from("hello")).call();






    // === Option<T>
    // The Option Enum: a value that can be something, or nothing.
    // Rust does not have NULLs: with this enum, you would have to handle both outcomes every time.
    // It's defined like this:
    //   enum Option<T> { // a generic
    //      None,
    //      Some(T),
    //      // `Some` and `None` are actually included in the prelude: you don't need to bring them into scope.
    //   }

    let some_number = Some(5); // Option<i32>
    let absent_number: Option<i32> = None;

    // Here's how to use it
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // ERROR: Rust doesn't know how to add `i8` and `Option<i8>` because they are different types.
    // To add them, you first need to convert `Option<i8>` into `i8` and handle the nullable case explicitly.






    // === Match expression
    // Match expression: it will run different code depending on which variant of the enum it has.
    // Match compares a value against a series of patterns: literal values, variable names, wildcards, ... 
    // See more in Chapter 18.

    // Coin matching: map to a value
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // Each arm is an expression that returns a value from `match` expression
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            // If you omit a variant, you will get this error:
            // > pattern `Coin::Quarter` not covered
        }
    }

    // Patterns can also bind values:
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    enum UsCoin {
        Penny,
        Nickel,
        Dime,
        // Until 2008, the US minted quarters with different designs for each of the 50 states on one side.
        // No other coins got state designs, so only quarters have this extra value.
        Quarter(UsState),
    }
    fn uscoin_value_in_cents(coin: UsCoin) -> u8 {
        match coin {
            UsCoin::Penny => 1,
            UsCoin::Nickel => 5,
            UsCoin::Dime => 10,
            // Add a variable for `state` to bind the value
            UsCoin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }






    //=== Matching with Option<T>
    // A function that takes an optional int, and if there's a value, adds 1:
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // Match and bind 
            Some(i) => Some(i+1),
        }
    }

    // Matches are exhaustive: the patterns must cover all possibilities. 
    // The compiler won't let you forget a variant.



    // Catch-all pattern:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // the pattern is a variable.
        // NOTE: put it last, because patterns are matched in order.
        other => move_player(other),
    }

    // Alternatively, use `_` to ignore the value and return a unit:
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // we don't need the value, and we won't run any code. Return the unit.
        _ => (),
    }






    // === if let
    let config_max = Some(3u8);

    // too wordy
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // more concise.
    // It takes a pattern and an expression and does the matching, but for only one arm.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // You can include an `else` with an `if let`
    let coin = UsCoin::Dime;
    if let UsCoin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        //...
    }
}


fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(_other: i32){}

```





# rust/a08_packages
## Packages, Creates, Modules, Paths

Module system includes:

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

A *crate* is the smallest amount of code that Rust compiler considers at a time.
A single source file is a crate. 

A *crate* can come in one of two forms:

* a *binary crate*: programs that compile into an executable. It must have a `fn main()`
* a *library crate*: define functionality to be shared with multiple projects.
  When Rustaceans say "create", they mean library crate.

The *crate root* is a source file that the Rust compiler starts from.
It makes up the root module of your crate.

A *package* is a bundle of one or more crates: it has a `Cargo.toml` file that describes 
how to build those crates. A package can contain many binary crates, but at most one library crate.

In `Cargo.toml`, there's no reference to `src/main.rs`: cargo follows a convention that this file is
the crate root of a binary crate, named after the package.

If a package contains both `src/main.rs` and `src/lib.rs`, it has two crates: a binary crate, and a library crate.
A package can have multiple binary crates by placing files in the `src/bin` directory: 
each file will be a separate binary crate.

### Modules Cheat Sheet

How modules work:

* When compiling a crate, the compiler first looks in the crate root file (usually `src/main.rs` or `src/lib.rs`).
* In the crate root file, you can declare new modules:
  
  ```rust
  mod garden;
  ```

  The compiler will look for the module's code in: `mod garden { ... }`, or in `src/garden.rs`, or in `src/garden/mod.rs`.

* A submodule can be defined as `mod vegetables { ... }`, or in `src/garden/vegetables.rs`, or in `src/garden/vegetables/mod.rs` (older style).
* Module code is private by default. Use `pub mod` to declare a module public.
* You can refer to submodule types as `crate::garden::vegetables::Asparagus`.
* Bring types into scope with `use crate::garden::vegetables::Asparagus` to reduce typing.






# rust/a08_packages/src


# rust/a08_packages/src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

// Tells the compiler to include garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

```



# rust/a08_packages/src/garden.rs

```rust
// Includes vegetables.rs
pub mod vegetables;

```





# rust/a08_packages/src/garden


# rust/a08_packages/src/garden/vegetables.rs

```rust
#[derive(Debug)]
pub struct Asparagus {}

```





# rust/a08_packages/src


# rust/a08_packages/src/lib.rs

```rust
// This is a library crate.
// The binary crate can use it as if it was an external library.
// See Chapter 12 for an example.

// Define a module.
// A module groups related definitions together.
mod back_office {
    // All items are private by default.
    // So, if you want to make an item private, you put it in a module.
    // That is, Rust is hiding inner implementation details by default.
    
    // Items in a parent module can't use the private items inside child modules.
    // However, items in child modules can use the items in their ancestor modules. 
    // This is because child modules wrap and hide their implementation details, 
    // but child modules can see the context in which they are defined.

    // Making a module public doesn't make the items public.
    // Every exported item must be marked with `pub` as well.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {
            // Use `super::` to start with the parent create: one level higher.
            super::play_music()
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    fn play_music(){}

    // If a struct is public, all fields are still private by default.
    // Use `pub` to make fields public.
    pub struct Order {
        pub id: u32,
    }
}


// Library's public API: marked with `pub`
pub fn eat_at_restaurant() {
    // Absolute path: starts with `crate::`
    crate::back_office::hosting::add_to_waitlist();

    // Relative path: starts with a module name
    back_office::hosting::add_to_waitlist();

    // Bring `hosting` into scope to reduce typing. It's just a shortcut.
    // It is idiomatic to bring the module into scope rather than just the function: 
    // module name makes it clear that it's not a local function.
    use back_office::hosting;
    hosting::add_to_waitlist();

    // With types, however, it is idiomatic to bring the type into scope directly:
    // because they have methods:
    use std::collections::HashMap;
    let mut map: HashMap<String, i32> = HashMap::new();

    // Use aliases to resolve name conflicts
    use std::fmt::Result;
    use std::io::Result as IoResult;  // alias
}

// Re-export: bring the name into scope and make it public.
pub use crate::back_office::Order;

// Use nested paths to bring several items into scope
use std::io::{self, Write}; // brings `io`, `Write` into scope

// Bring all public items into scope:
use std::collections::*;

```





# rust/a09_collections/src


# rust/a09_collections/src/main.rs

```rust
#[allow(unused_variables)]
fn main() {
    // === Vector === //
    // Vector: Vec<T> is a list of items of the same type, stored in the heap, next to each other in memory.
    let v: Vec<i32> = Vec::new();

    // Empty vector needs a type.
    // When values are provided, the compiler will infer the type from the values:
    let mut v = vec![1, 2, 3]; // macro

    // Add elements
    // NOTE: don't forget to make the value mutable!
    v.push(4);

    // Get element
    let third: &i32 = &v[2];  // Will panic if index is out of bounds
    let third: Option<&i32> = v.get(2); // Will not panic, but return an Option<T>
    match third {
        Some(third) => println!("Third={third}"),
        None => println!("Third=<none>"),
    }

    // Iterate
    for i in &mut v { // iterate mutable: make changes
        *i += 50;
    }
    for i in &v { // iterate immutable: just read
        println!("i={i}");
    }

    // Vectors can store values that are the same type.
    // Let's use enums to store different value types:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];






    // === String === //
    // `str` literals are stored in the program's binary.
    // `String` are growable, mutable, owned, UTF8-encoded.
    // Strings are implemented as a vector of bytes + methods used when those bytes are interpreted as text.
    let s = "initial string".to_string();
    let s = String::from("initial_string");

    // Append a string
    let mut s = String::new();
    s.push_str("Hello");

    // Append a byte, a single character
    s.push('!');

    // Concat strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // NOTE: `s1` has been moved here, `s1` can no longer be used
                        // NOTE: `&s2` because of the method signature: `fn add(self, s: &str)`

    // Format: add multiple strings together
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Indexing. Because of UTF-8, you can only use ranges:
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // takes the first 4 bytes: 2 letters. It will panic if a letter is broken.

    // Iterate over characters
    for c in "Зд".chars() { // iterate: characters
        println!("{c}");
    }
    for b in "Зд".bytes() { // iterate: bytes
        println!("{b}");
    }






    // === Hash Maps === //
    // HashMap.
    // Uses `SipHash` by default: it's slow, but resistant to DoS attacks with hash tables.
    // Use a faster "hasher" if you like: a type that implements the `BuildHasher` trait.
    use std::collections::HashMap;

    // Create a hash map
    let mut scores = HashMap::new();

    // Add values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Lookup a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get()       -> Option<&i32>
    // .copied()    -> Option<i32>
    // .unwrap_or() -> i32

    // Iterate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // If a value is already there: 1) overwrite
    scores.insert(String::from("Blue"), 25);

    // If a value is already there: 2) insert only if not present
    scores.entry(String::from("Blue")).or_insert(50);

    // If a value is already there: 3) update
    let text = "hello world wonderful world";
    let mut wordcount = HashMap::new();
    for word in text.split_whitespace() {
        let count = wordcount.entry(word).or_insert(0);
        *count += 1;
    }
}

```





# rust/a10_errors


# rust/a10_errors/Cargo.toml

```toml
[package]
name = "a10_errors"
version = "0.1.0"
edition = "2021"

[profile.release]
# Default panic!() behavior: unwinding:
# that is, Rust walks back up the stack and cleans up the data from each function.
# This is expensive. Alternative: immediate aborting, with no cleaning up.
# To be used when you need a binary to be as small as possible.
panic = 'abort'

[dependencies]

```





# rust/a10_errors/src


# rust/a10_errors/src/main.rs

```rust
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

```





# rust/a11_generics_traits_lifetimes/src


# rust/a11_generics_traits_lifetimes/src/main.rs

```rust
use std::fmt::Display;
use std::fmt::Debug;


#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // A function that finds the largest item in the slice
    // The <T> must be a comparable value: have a trait that allows comparisons
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    // A generic struct
    struct Point<T> {
        x: T,
        y: T,
    }

    // A generic method
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Constraints: we can implement methods only on Poing<f32> instances
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };








    // === Traits === //
    // A trait: similar to interfaces in other languages.
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Implement a trait
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {  // implements a trait
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }


    // A trait with a default behavior.
    // It will be overridden by specific implementationa
    pub trait Summary2 {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    impl Summary2 for NewsArticle {} // use the default implementation


    // A function that has a trait as a parameter:
    // it will accept any object that has Summary implemented.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // `&impl Summary` parameter is actually synax sugar for a longer form:
    // the "trait bound":
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Specify multiple trait bounds
    pub fn notify3(item: &(impl Summary + Display)) {}
    pub fn notify4<T: Summary + Display>(item: &T) {}

    // Trait bounds can get very long. That's why Rust has an alternate syntax:
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        10
    }

    // Return a type that implements traits
    fn returns_summarizable() -> impl Summary {
        NewsArticle{
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("content"),
        }
    }

    // NOTE: traits are like generics: they compile a version of this function for a type.
    // This means that you cannot implement a function that returns objects of different types this way.
    // To have a value hold values of different types, see "Trait Objects"


    // Trait bounds for methods: conditionally implement methods for objects that have a trait
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T: Display + PartialOrd> Pair<T> {  // only for types that implement a trait
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Blanket implementation:
    // implement a method for *any* type that satisfies the trait bounds.
    // This is how standard library has the `.to_string()` method on any type that implements the `Display` trait:
    impl<T: Display> ToString for T {
        // --snip--
    }






    // === Lifetimes === //
    // Lifetime: a kind of generic. It ensures that references are valid as long as we need them to be.
    // Every reference has a "lifetime": the scope for which that reference is valid.
    // Most of the time, lifetimes are implicit or inferred.

    // The main aim of lifetimes is to prevent "dangling references".

    // Rust has no values. But Rust won't let you use this variable unless you initialize it.
    let r;

    // `x` only lives inside this scope. So it we wanted to do `r = &x`, it would fail: `x` doesn't live long enough.
    // `r` lives longer, though, because it's defined in the outer scope.
    {
        let x = 5;
        r = &x;  // will fail
    }
    println!("r: {}", r);


    // This will not fail:
    let x = 5;
    let r = &x;


    // Let's write a function that returns the longest of two strings.
    // We don't want it to take ownership, so its parameters are references.

    // This won't compile because Rust doesn't know whether the return value referes to `a` or `b`.
    // Actually, we don't know either: it's dynamic. So the borrow checker won't be able to evaluate lifetimes.
    //      fn longest(a: &str, b: &str) -> &str {
    // We need to add a lifetime annotation that expresses the following constraint:
    // the returned reference will be valid as long as both the parameters are valid.
    // The lifetime annotations become part of the contract of the function.
    // The returned reference's lifetime will be equal to: the smallest of the lifetimes of `a` and `b`
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }

    // A lifetime annotation is placed after the `&` reference.
    // It's just a named marker. They are used to describe the relationships of the lifetimes of multiple references.
    let v: &'a i32 = 10;
    let v2: &'a mut i32 = 20; // has the same lifetime!



    // So far, our structs had owned types.
    // When a struct holds references, you'd need to add a lifetime annotation on every reference:
    struct ImportantExcerpt<'a> { // <'a> means the struct can't outlive the `part` field value's reference
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // In Rust, every reference has a lifetime.
    // In many cases, the compiler can infer lifetimes using "lifetime elision rules".
    // If the compiler is not able to resolve an ambiguity, you will need to write the lifetimes explicitly.

    // "Input lifetimes": lifetimes on function or method parameters
    // "Output lifetimes": lifetimes on return values

    // Rules:
    // 1. Every reference parameter gets a separate lifetime parameter
    // 2. If there is exactly 1 input lifetime parameter, it is assigned to all output lifetime parameters
    // 3. If one parameter is `&self` (it's a method), the lifetime of `self` is assigned to all output lifetimes.




    // The 'static lifetime: this reference can live for the entire duration of the program.
    // All string literals are static.
    let s: &'static str = "I have a static lifetime";






    // All generics together:

    fn longest_with_an_announcement<'a, T>( // func, with <'a> lifetime generic, with <T> generic
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where // trait bound
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }



    // More examples on lifetimes:
    // https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
}

```





# rust/a12_tests/src


# rust/a12_tests/src/lib.rs

```rust
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

```





# rust/a12_tests/tests


# rust/a12_tests/tests/common.rs

```rust
pub fn setup(){
    // common setup code
}
```



# rust/a12_tests/tests/integration_test.rs

```rust
// Rust will compile each file under ./tests as an individual crate.
// Unlike sub-module tests, this test can only use the external API of the library: `pub` functions.


use a12_tests;

pub mod common;
// use common; // use a submodule

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, a12_tests::add(2, 2));
}
```





# rust/a13_cli_app/src


# rust/a13_cli_app/src/lib.rs

```rust
// has: Files & filesystem
use std::fs;

// has: Error
use std::error::Error;

// Logic
// Return value: OK unit, or "trait object" `Box<dyn Error>`: any type that implements the `Error` trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Open the file for reading
    // Don't `.expect()`: rather, return errors with the `?` operator.
    let contents = fs::read_to_string(config.file_path)?;

    // Search
    for line in search(&config.pattern, &contents) {
        println!("{line}");
    }

    Ok(())
}

// Search: pattern in file
// Lifetime `'a`: the result will live as long as the `contents` argument
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Search
    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }

    // Done
    results
}


// Config methods
impl Config {
    // Config builder
    // The returned error string is `'static` because we actually return a static (i.e. literal) string.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < (2+1) {
            return Err("not enough arguments");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{
            pattern,
            file_path,
        })
    }
}

// App config: command-line input
#[derive(Debug)]
pub struct Config {
    pattern: String,
    file_path: String,
}



// Unit-test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
";
        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }
}
```



# rust/a13_cli_app/src/main.rs

```rust
// has: Environment and arguments
use std::env;

// has: Exit()
use std::process;

// Our lib: we use `Config`, `run()`
use a13_cli_app as minigrep;

fn main() {
    // Get cmdline arguments.
    // NOTE: it will panic if argument contains invalid Unicode.
    // To accept invalid unicode, use `args_os()` instead.
    let args: Vec<String> = dbg!(env::args().collect());
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        // `eprintln!()` prints to stderr
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        process::exit(255);
    });
    println!("config={config:?}");

    // Logic moved into lib.rs
    // Run it, handle errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```





# rust/a14_functional/src


# rust/a14_functional/src/main.rs

```rust
use std::collections::HashMap;

// === Closures.
// Closures capture values from their scope and can use them elsewhere,
// even it evaluated in a different context.

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}


// Implement a store of shirts, with a list of shirts in stock
struct StoreInventory {
    // The colors the store currently has in stock
    shirts: Vec<ShirtColor>
}
impl StoreInventory {
    // The user can choose a shirt color to get as a present.
    // If the user didn't specify a color, we give away a random shirt.
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Take the user's color, or run a closure: picks the most stocked shirt.
        // `.unwrap_or_else<T>()` receives a closure. Unlike `unwrap_or()`, which receives a value.
        // This closure, with no parameters (`||`) has access to `&self`, and returns ShirtColor.
        // It has no `{ ... }` because it simply returns a value from an expression.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    let store = StoreInventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // Don't give a preference. The algorithm will pick a shift with more in stock
    let free_shirt = store.giveaway(None);
    println!("Getting color: {free_shirt:?}");



    // Closures typically don't have type annotations, but you still can add them for explicitness.
    // Without type annotations, the compiler infers the closure's type by the caller context (!!)
    //
    // In this definition, the closure doesn't have any parameter type ... yet.
    let just_return = |x| x;
    // But when we use it, it gets a type:
    let _s = just_return(String::from("hey")); // now the closure has `x: String`

    // Closures can capture values from their environment in three ways:
    // 1) borrowing immutably
    // 2) borrowing mutably
    // 3) taking ownership
    // The closure will decide which of these to use based on
    // what the body of the function does with the captured values.
    let mut list = vec![1, 2, 3];
    let _ = || println!("value: {:?}", list);  // immutable borrow is sufficient
    let _ = || list.push(7); // mutable borrow is necessary
    let _ = move || list; // taking ownership: explicitly requested by the `move` keyword

    // Example of taking ownership: move the value into a thread
    use std::thread;
    let mut list = vec![1, 2, 3];
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();



    // Once the closure captures a reference/ownership, the body of the function defines what happens:
    // 1) move a captured value out of the closure
    // 2) mutate the captured value
    // 3) neither move nor mutate
    // Closures will automatically implement one, two, or three of these traits:
    // * `FnOnce`: closures that can only be called once.
    //             When used as a trait bound, it says "we're going to call this function once".
    //             Applies to functions that move a value out of the environment: can only be done once.
    // * `Fn`:     closures that don't move the captured values out of their body
    //             and that don't mutate captured values.
    //             It also applies to closures that capture nothing from their environment.
    //             This is important when a closure may be called concurrently.
    // * `FnMut`:  closures that don't move captured values out of their body,
    //             but that might mutate the captured values.
    //             This trait is used when a closure is going to be called multiple times

    // Another explanation:
    // With closures as input parameters, the closure's complete type must be annotated:
    // * `Fn` the closure uses the captured value by reference (`&T`)
    // * `FnMut` the closure uses ehe captured value by mutable reference (`&mut T`)
    // * `FnOnce` the closure uses the captured value by value
    // The compiler will capture variables in the least restrictive manner possible.

    // Example: a list of rectangles
    let mut list = [
        Rectangle{ width: 9, height: 1},
        Rectangle{ width: 3, height: 5},
        Rectangle{ width: 7, height: 8},
    ];

    // This closure implements the `FnMut` trait:
    // because it is called multiple times. It doesn't capture or mutate, though.
    list.sort_by_key(|r| r.width);

    // This closure implements just the `FnOnce` trait:
    // because it moves a value out of the environment (the `value` is moved)
    let mut seen_items = vec![];
    let value = String::from("hey");
    list.sort_by_key(|r| {
        // move `value` out. This makes the closure `FnOnce`: it cannot be called multiple times.
        // Error: "cannot move out of `value`, a captured variable in an `FnMut` closure"
        seen_items.push(value); // this fails
        // To fix this, we need to make sure we don't move values out of the environment.
        seen_items.push(value.clone()); // this works

        r.width
    });




    // === Iterators
    // In Rust, iterators are lazy: they have no effect until you call methods
    // to consume the iterator to use it up.
    let v1 = vec![1, 2, 3];

    // `.iter()` creates an iterator that returns immutable references to the values in the vector.
    // `.into_iter()` creates an iterator that takes ownership of `v1` and returns owned values
    // `.iter_mut()` creates an iterator over mutable references
    let v1_iter = v1.iter(); // store the iterator
    for value in v1_iter {  // use the iterator. The loop take ownership and makes it mutable (!)
        println!("Iterated value: {value}");
    }

    // Iterators implement the `Iterator` trait with `.next()` method:
    // * it returns one item at a time, as Option<T>.
    // * it returns `None` when the iterator is "consumed" (or "used up")

    // "Consuming adaptors": methods of an iterator that *consume* the iterator
    // The `sum()` method of an iterator takes ownership of the iterator, consumes it, adds to a running total:
    let total: i32 = v1.iter().sum();
    println!("Sum: {total}"); //-> 6

    // "Iterator adaptors": methods of an iterator that don't produce a different iterator.
    // The `.map()` method produces an iterator with modified values:
    // We use `.collect()` to form a collection.
    let incremented: Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("Incremented: {:?}", incremented); //-> [2, 3, 4]

    // Filter: only return values that match the condition
    let odd: Vec<&i32> = v1.iter().filter(|x| *x%2==0).collect(); // dereference `*x` to convert `&int32` -> `int32`
    println!("Odd: {:?}", odd); //-> [2]
}


// --- helpers


// This method is of no significance for closure study, but we still need it implemented.
// But it illustrates how Rust allows us to have multiple `impl` blocks.
impl StoreInventory {
    // Find a shirt that we have the most of.
    fn most_stocked(&self) -> ShirtColor {
        // NOTE: this is my code, it may be bad
        // Count
        let mut counts: HashMap<ShirtColor, i32> = HashMap::new();
        for color in &self.shirts {
            counts.insert(
                *color, // clones the value
                counts.get(color).unwrap_or(&0) + 1
            );
        }

        // Now find the largest one
        let mut max_color: Option<ShirtColor> = None;
        let mut max_color_count: i32 = 0;
        for (color, count) in &counts {
            if count > &max_color_count {
                max_color = Some(*color);
                max_color_count = *count;  // clones the value
            }
        }

        // TODO: it will panic if the stock is empty
        max_color.unwrap()
    }
}


// A rectangle
struct Rectangle {
    width: u32,
    height: u32,
}
```





# rust/a15_cargo/src


# rust/a15_cargo/src/lib.rs

```rust
//! # My crate
//!
//! This is a doccomment for the file
//! It adds documentation to the item that contains it: file (the crate) or the module.

/*! The so-called "inner doc comment" (for the crate)
 * can also look like this
 */

/// Doccomments. Start with triple /
///
/// # Examples
/// Here we go with Markdown.
/// This Markdown will be used on crates.io to document the usage of your library.
///
/// This code will actually be run as a unit-test:
///
/// ```rust
/// # // line prefixed with `#` is ignored in the documetation, but is still tested!
/// let arg = 5;
/// let answer = a15_cargo::add_one(arg);
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
/// never panics
///
/// # Errors
/// Describe the kinds of errors that might occur
///
/// # Safety
/// If the function is `unsafe` to call, explain it
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** This is another type of "outer doc comment":
 * It documents a function while being outside of it
*/
pub fn something(){}




// So:
//  //!      Inner doc comment
//  /*! */   Inner doc comment
//  ///      Outer doc comment
//  /**  */  Outer doc comment




// We can generate HTML from this documentation into `target/doc` using
// $ cargo doc
// To open it in the browser:
// $ cargo doc --open

// Note: `cargo test` will run the code examples in the documentation as tests!
// $ cargo test



// A published module
pub mod artistic {
    //! Supplementary features

    /// Primary colors in the RYB model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}

// Re-export a type/function for convenience
pub use self::artistic::PrimaryColor;
```





# rust/a15_cargo


# rust/a15_cargo/Cargo.toml

```toml
# crate medata
# used when publishing to crates.io:
# $ cargo publish
[package]
name = "a15_cargo"
version = "0.1.0"
edition = "2021"

license = "MIT"  # open-source :)
description = "A crate that does nothing"

# "Release profiles": different configurations for compiling code:
# * "dev": for `cargo build`.
# * "release": for `cargo build --release`.
[profile.dev]
# The number of optimizations Rust will apply to the code.
# More optimizations extends compiling time.
opt-level = 0  # the default

[profile.release]
opt-level = 3  # the default




# A "workspace" is a set of packages that share the same `Cargo.lock` and output directory.
# It you to split one large lib.rs into smaller packages.
# Run this to create a binary sub-crate:
# $ cargo new adder --lib
#
# The workspace has one target directory.
# Compiled artifacts will end up in ./target
[workspace]
members = [
    # sub-library.
    # Generate it with:
    # $ cargo new adder --lib
    "adder"
]



# Dependencies
[dependencies]
# Use our sub-library as a dependency :)
adder = { path = "./adder" }


# Dependencies only for development
[dev-dependencies]
# Extends standard `assert_eq!()` with colorful diff
pretty_assertions = "1"
```





# rust/a15_cargo/adder/src


# rust/a15_cargo/adder/src/lib.rs

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```





# rust/a15_cargo/src


# rust/a15_cargo/src/main.rs

```rust
use adder;

fn main() {
    // Use our library
    let x = adder::add_one(1);
    println!("Using a library: {x}");
}

```





# rust/a16_smart_pointers/src


# rust/a16_smart_pointers/src/main.rs

```rust
fn main() {
    // Smart pointers: data structures that act like a pointer, but also have additional capabilities.
    // E.g. `Rc`: reference counting smart pointer that allows data to have multiple owners.

    // In many cases, smart pointers *own* the data they point to.
    // `String` and `Vec<T>` count as smart pointers because they own some memory
    // and allow you to manipulate it.
    // As for extra capacities: `String` stores its capacity, and has the ability to ensure valid UTF-8;

    // Smart pointers implement `Deref` and `Drop` traits:
    // * `Deref` allows an instance of the smart pointer (usually a struct) to behave like a reference
    // * `Drop` allows you to customize the code that's run when an instance goes out of scope (destructor?)

    // Many libraries have their own smart pointers, but here are the most common ones:
    // * `Box<T>` for allocating values on the heap
    // * `Rc<T>` a reference counting type that enables multiple ownership
    // * `Ref<T>` and `RefMut<T>`: enforces the borrowing rules at runtime instead of compile time.
    //   They are accessed through `RefCell<T>`
    // * "Interior mutability": when an immutable type exposes and API for mutating an interior value


    // An overview:
    // * `Rc<T>` enables multiple owners of the same data;
    //   `Box<T>` and `RefCell<T>` have single owners.
    // * `Box<T>` allows immutable or mutable borrows checked at compile time;
    //   `Rc<T>` only allows immutable borrows checked at compile time;
    //   `RefCell<T>` allows immutable or mutable borrows checked *at run time*.
    // * With `RefCell<T>`, you can mutate the value inside it even when the `RefCell<T>` is immutable.




    // === Box<T>
    // Boxes allow you to store data on the heap rather than the stack. The stack will only have a pointer to the heap.
    // Boxes don't have performance overhead (other than storing their data on the heap).
    // Use boxes when:
    // * When you have a type whose size can't be known at compile time
    // * When you have a large amount of data and want to transfer ownership without copying data
    // * When you want a value of any type, but with a specific trait

    // Store this `i32` on the heap.
    // When the owned Box goes out of scope, it will be deallocated.
    let _ = Box::new(5 as i32);  // example: type casting

    // Boxes enable "recursive types": a type that can have another value of the same type as part of itself.
    // Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.
    // Example: cons list. It comes from Lisp, and is a version of a linked list:
    // each item contains 2 elements: [the value of the current item, next item]. Recursion ends with `Nil`.
    //  (1, 2, (3, Nil))

    enum List {
        // This naive example will fail: "recursive type `List` has infinite size": "recursive without indirection"
        // This is because Rust can't figure out how much space it needs to store a `List` value.
        Cons(i32, List),  //FAILS

        // The corrected version.
        // It works because the `Box<T>` pointer takes const space no matter what data is in there.
        Cons(i32, Box<List>),

        Nil,
    }
    use List::{Cons, Nil}; // import nested `Cons` and `Nil` into this scope

    let _ = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );



    // The "dereference operator" `*`:
    let x = 5;
    let y = &x; // a reference: a pointer to a value stored elsewhere
    assert_eq!(5, *y); // get the value: "dereference"
    assert_eq!(5, y); // Error: help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`


    // Boxes implement the `Deref` trait, which allows `Box<T>` values to be treated like references.
    // The `Deref` trait allows you to customize the behavior of the "dereference operator" `*`.
    // Using `Box<T>` like a reference:
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);  // dereference



    // Let's build our own smart pointer: to see how it works.
    struct MyBox<T>(T); // a tuple

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {  // implement the `Deref` trait
        // Defines an associated type for the `Deref` trait to use: it's like a generic parameter
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // And use it
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);  // dereference. Behind the scenes: `*(y.deref())`



    // The `Drop` trait: run code when a value is about to go out of scope.
    // The compiler will insert this code automatically.
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // Rust won't let you call `.drop()` manually because it will run this function itself. It'd be a double-free.
    // Use `std::mem::drop()` function if you must:
    let c = CustomSmartPointer{
        data: String::from("hey"),
    };
    drop(c);






    // === Rc<T>
    // Reference-counting smart pointer.
    // Used in cases where a single value might have multiple owners:
    // E.g. in graph data structures, where multiple edges might point to the same node:
    // a node shouldn't be cleaned up unless it doesn't have any edges pointing to it and so has no owners.

    // The `Rc<T>` type keeps track of the number of references to a value to determine
    // whether or not the value is still in use. The value is cleaned up when there are 0 references.

    // Example: a person enters the TV room and turn it on. Others come in and join: the TV is still on.
    // When the last person leaves the room, they turn the TV off: it's no longer being used.
    // But if someone turns the TV off while others are still watching, there would be an uproar!! :)

    // ❗ NOTE: `Rc<T>` is only for use in single-threaded scenarios!!
    // See Chapter 16: using `Rc<T>` in multithreaded systems.

    // Let's implement a value that can be added to two lists:
    let two = Box::new(2); // a value
    let _list_a = vec![Box::new(1), two, Box::new(3)];
    let _list_b = vec![Box::new(1), two, Box::new(3)];  // ERROR: "use of moved value: `two`"

    // it fails because `list_a` owns the value of `two`: no one else can own it
    use std::rc::Rc;
    let two = Rc::new(2); // a value
    let _list_a = vec![Rc::new(1), Rc::clone(&two), Rc::new(3)];
    let _list_b = vec![Rc::new(1), Rc::clone(&two), Rc::new(3)];  // Rc::clone() increments the reference count

    // NOTE: two.clone() is also ok, but for most types, it creates a deep copy.
    // The convention is to use Rc::clone() that always makes a shallow copy. It's cheaper.

    // See the reference count
    println!("RC count for `two`: {}", Rc::strong_count(&two));  //-> 3

    // NOTE: `Rc<T>` returns immutable references. You cannot modify the value. It's read-only.
    // For mutable refrences, see `RefCell<T>`






    // === RefCell<T>
    // "Interior mutability" allows you to mutate data even when there are immutable references to that data.
    // This is normally disallowed, but this pattern uses `unsafe` code to bend the usual rules:
    // code marked as `unsafe` promises to check the rules manually.
    // So: with `RefCell<T>`, you can mutate the value inside it even when the `RefCell<T>` is immutable.

    // This pattern is useful when the conservative compiler doesn't let you do something that you're sure
    // you can safely handle yourself. The compiler is "conservative" in that it would rather reject
    // a correct program than allow an incorrect one. The `RefCell<T>` lets you bend the rules.

    // Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds.
    // This means that:
    // * At any given time, you can *either* have one mutable reference, or many immutable references
    // * References must always be valid

    // With `Box<T>`, the borrowing rules are enforced at compile time;
    // whereas with `RefCell<T>` the borrowing rules are enforced *at runtime*.
    // If you break the `RefCell<T>` rules, your program will panic!

    // The internal code of `RefCell<T>` is `unsafe`, but the API is safe.
    // ❗ NOTE: `RefCell<T>` is only for use in single-threaded scenarios!!
    // ❗ See `Mutex<T>`: the thread-safe version of RefCell<T>


    // A use case for interior mutability: Mock Objects.
    // Let's first implement an object, `LimitTracker`, that checks how close a user is to a quota:

    /// Notifies the user: SMS or something
    pub trait Messenger {
        /// Send a notification, e.g. SMS
        fn send(&self, msg: &str);  // takes an immutable reference to `&self`, and to the text
    }

    /// Limit checker: if a user is close to a limit, send a notification.
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: u64,
        max: u64,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        /// Constructor
        pub fn new(messenger: &'a T, max: u64) -> LimitTracker<'a, T> {
            LimitTracker{messenger, value: 0, max}
        }

        /// Set the new value.
        /// Notify the user if they're close to the quota.
        pub fn set_value(&mut self, value: u64){
            // Set the value
            self.value = value;

            // Usage
            let perc = self.value as f64 / self.max as f64;

            // Check quota
            if perc >= 1.0 {
                self.messenger.send("Error: over quota!");
            } else if perc >= 0.9 {
                self.messenger.send("Warning: quota almost used up!");
            }
        }
    }

    // Now, the mock object.
    // It needs to implement the trait.
    use std::cell::RefCell;

    struct MockMessenger {
        // The interface defines `send(&self)` as an immutable reference.
        // But we need a mutable reference that will let us modify the value.
        // This is where interior mutability can help!
        // We'll store `sent_messages` within a RefCell<T>, which is itself immutable,
        // but the value that it points to is mutable.
        sent_messages: Vec<String>,  // error: "`self` is a `&` reference, so the data it refers to cannot be borrowed as mutable"
        sent_messages: RefCell<Vec<String>>, // OK
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                // Fails
                sent_messages: vec![],  //FAILS
                // Works
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Fails
            self.sent_messages.push(String::from(message)); //FAILS
            // Works.
            // `.borrow_mut()` gets a mutable reference, which is returned when dropped
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }


    // Use it
    let messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&messenger, 100);

    limit_tracker.set_value(95);
    // `.borrow()` gets an immutable reference
    assert_eq!(messenger.sent_messages.borrow().len(), 1);


    // Here is how it works:
    // the `RefCell<T>` keeps track of how many `.borrow()` (`Ref<T>`) and `.borrow_mut()` (`RefMut<T>`)
    // smart pointers are currently active. When a value gets out of scope (dropped), the count decrements.
    // It lets you have many immutable borrows, or one mutable borrow.
    // If you try to violate the rules, it panics.






    // === Having multiple owners of mutable data
    // Recall: `Rc<T>` lets you have multiple owners of immutable data.
    // Recall: `RefCell<T>` is immutable. So you can share multiple references.
    // By combining `Rc<T>` and `RefCell<T>`, you can share mutable data by allowing multiple references!

    // Two lists will point to a shared mutable value
    let two = Rc::new(RefCell::new(2));
    let list_a = vec![
        Rc::new(RefCell::new(1)),
        Rc::clone(&two),
        Rc::new(RefCell::new(3)),
    ];
    let list_b = vec![
        Rc::new(RefCell::new(1)),
        Rc::clone(&two),
        Rc::new(RefCell::new(3)),
    ];

    // Mutate it
    *two.borrow_mut() += 10;

    // Check
    println!("list_a={:?}", list_a);  //-> list_a=[RefCell { value: 1 }, RefCell { value: 12 }, RefCell { value: 3 }]
    println!("list_b={:?}", list_b);  //-> list_b=[RefCell { value: 1 }, RefCell { value: 12 }, RefCell { value: 3 }]






    // === Weak references
    use std::rc::Weak;
    // `Rc<T>` may form reference cycles that are never cleaned up: rc count never drops down to zero.
    // To prevent reference cycles, turn `Rc<T>` into a `Weak<T>`:
    // use `Rc::downgrade()` to get a weak reference. It does not express an ownership relationship:
    // their count does not affect when an `Rc<T> instance is cleaned up.
    // A weak reference increases the `weak_count` of an `Rc<T>`. They do not prevent cleaning up.

    // To get a value from a weak reference, call its `.upgrade()` method to get an `Option<Rc<T>>`:
    // it returns a reference, or `None` if the value has been dropped already.

    // Let's create a tree whose items know about their children *and* their parents.
    #[derive(Debug)]
    struct Node {
        // Node value
        value: i64,

        // Mutable reference to a list of children, where every child is represented as an `Rc<Node>`.
        // When creating this object, we will need to set the children, and then put the same values into parents.
        // To allow that, we need:
        // * `Rc<Node>` to share ownership (with the calling code), and to allow weak references
        // * `RefCell<Node>` to make mutable
        //
        // To allow that, we need
        children: RefCell<Vec<Rc<Node>>>,

        // Parents: weak references to Nodes
        // ❗ We cannot use strong references because that would create a reference cycle
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    let branch = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),  // leaf Rc
        parent: RefCell::new(Weak::new()),
    });

    // update the parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

}

```





# rust/a17_concurrency/src


# rust/a17_concurrency/src/main.rs

```rust
use std::thread;
use std::time::Duration;


fn main() {
    //=== Threads
    // Rust stdlib uses a 1:1 model of thread implementation: one OS thread per one language thread.
    // Why? Because the M:N model requires a runtime, and Rust, being a systems programming language, shouldn't have one.
    // There are crates that implement other models of threading.

    // `thread::spawn()` starts a closure in a thread.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // `.join()` waits for it to complete.
    // Note that when main() quits, all threads are shut down.
    handle.join().unwrap();

    // The `move` keyword is often used with thread closures: it will take ownership of the values it uses.
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector: {v:?}");
    });
    handle.join().unwrap();


    // Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems
    // were two separate challenges to be solved with different methods.
    // Over time, the team discovered that the ownership and type systems are a powerful set of tools
    // to help manage memory safety *and* concurrency problems!






    // === Messages-Passing
    // Message-sending concurrency: "do not communicate by sharing; share by communicating!"
    // A "channel" is a concept by which data is sent from one thread to another.
    // It has two halves: a transmitter, and a receiver.
    // A channel is said to be "closed" if either the transmitter or receiver half is dropped.

    // MPSC: Multiple-Producers, Single-Consumer channel.
    // It can have multiple senders, but only one receiver.
    use std::sync::mpsc;

    // Get a pair of channels: a tuple (destructure)
    let (tx, rx) = mpsc::channel();

    // Start multiple producers
    for _ in 0..5 {
        // Clone the transmitter
        let tx = tx.clone();

        // Spawn a thread, move the value
        thread::spawn(move || {
            // Send the value.
            // Note: the channel takes ownership of the value. The receiving end will take it afterwards.
            let val = String::from("hey");
            tx.send(val).unwrap();
        });
    }

    // Receive here
    let received = rx.recv().unwrap();
    println!("From channel: {:?}", received);

    // Receive more. Stop when the channel is closed.
    // NOTE: we don't have to wait for the threads to finish: consuming the channel will wait until they're all done!
    drop(tx);  // close the last remaining reference. Otherwise we'll block forever.
    for msg in rx {
        println!("Received: {}", msg);
    }






    // === Shared-State Concurrency
    // "Mutex" only allows one thread to access some data at any given time. The mutex "guards" the data.

    use std::sync::Mutex;
    let m = Mutex::new(5); // the guarded value

    // We start a scope.
    // When the scope ends, the mutable reference is dropped, and the mutex is auto-unlocked!
    {
        // Get a mutable reference.
        // `.lock()` will fail if another thread holding the value panicked. No one would able to get the lock.
        // So we use `.unwrap()` to have this thread panic in this situation.
        let mut num = m.lock().unwrap();

        // The type system ensures that you acquire the lock before using the value:
        // you've got to call the `.lock()` method to get the `Mutex<T>` type, which implements `Deref`.
        // It also implements `Drop` that releases the lock automatically when it goes out of scope.
        // This is how borrowing rules + type system remove the risk of forgetting to release the lock :)
        // Now mutate the value:
        *num += 1;
    }

    // Let's share it between threads.
    // We cannot clone the mutex: it won't work.
    // We cannot use `Rc<Mutex<...>>` because `Rc<T>` is not thread-safe. Compiler will give us an error.
    // We need to use `Arc<T>`: atomic reference count.
    use std::sync::Arc;

    let counter = Arc::new(Mutex::new(0)); // shared between threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            // Lock, check, mutate
            *counter.lock().unwrap() += 1
        });
    }

    // Give the threads some time.
    // We could use `.join()` instead.
    thread::sleep(Duration::from_millis(100));

    // See the result
    println!("Result: {}", *counter.lock().unwrap());  //-> 10






    // === Extensible concurrency: the `Sync` and `Send` traits.

    // The `Send` marker trait indicates that ownership of self can be transferred between threads.
    // Almost every Rust type is `Send`.
    // Any type composed entirely of Send types is automatically marked as Send as well.
    // `Rc<T>` is not `Send`: it's not thread-safe.
    use std::marker::Send;

    // The `Sync` marker trait indicates that it is safe for the type to be referenced from multiple threads.
    // Any type `T` is `Sync` if `&T` is `Send`: i.e. an immutable reference to `T` can be sent.
    // Primitive types are `Sync`, and types composed entirely of `Sync` types are also `Sync`.
    // `Rc<T>` is not `Sync`: it's not thread-safe.
    // `Mutex<T>` is `Sync`.
    use std::marker::Sync;

    // Note: a "marker trait" is a trait with no methods to implement.
}


```





# rust/a18_oop/src


# rust/a18_oop/src/main.rs

```rust
fn main() {
    // Rust is object oriented, in a way:
    // structs and enums have data, and `impl` blocks provide methods on them.
    // Methods in Rust can be `pub`: this is the only public API that users can see.

    // Rust has no inheritance.
    // For code reuse, some limited inheritance is available with default trait methods.

    // Inheritance has recently fallen out of favor as a programming design solution:
    // it's all too easy to share more code than necessary. In addition, some languages
    // only allow single inheritance, which is also quite limited.
    // For these reasons, Rust uses trait objects instead of inheritance.

    // "Trait objects" allow for values of different types: it's an interface.
    // The concept of Rust is: "duck typing". The interface matters, the type -- not as much.
    // Define a trait:
    pub trait Draw {
        fn draw(&self);
    }

    // Define a struct
    pub struct Screen {
        // A list of references to "trait objects": any object that implements the trait.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self){
            for component in self.components.iter() {
                // Trait objects peform "dynamic dispatch": the compiler can't tell which method that is,
                // so it emits code that figures out the method to call at runtime.
                // This lookup incurs a runtime cost. It also prevents the compiler from inlining the method.
                component.draw();
            }
        }
    }

    // Implement a `Draw` type
    pub struct Button {
        pub w: u32,
        pub h: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self){
            // ... draw code
        }
    }

    // Now we can build the GUI:
    let screen = Screen {
        components: vec![
            Box::new(Button{w: 75, h: 10, label: String::from("OK")}),
        ],
    };
    screen.run();

}

```





# rust/a19_match/src


# rust/a19_match/src/main.rs

```rust
fn main() {
    // "Patterns" are for matching against the structure of types:
    // * Literals
    // * Destructured arrays
    // * Variables
    // * Wildcards
    // * Placeholders
    // Patterns describe the *shape* of data.

    let x = Some(1);

    // Match expression.
    // A match expression must be "exhaustive": i.e. all possibilities must be accounted for
    let _y = match x {
        // Match arms
        None => 0,
        Some(i) => i + 1,

        // Default match-all:
        _ => 0,
    };


    // `if let` expressions: can match only one value to an expression
    let favorite_color: Option<&str> = None;

    if let Some(color) = favorite_color {
        println!("Your favorite color: {color}");
    }


    // `while let` conditional loop: run a loop for as long as the pattern continues to match
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Top: {top}");
    }



    // `for` loops can match a pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("#{index}: {value}");
    }




    // `let` can match a pattern
    let (x, y, z) = (1, 2, 3);



    // Function parameters can also be patterns
    fn print_coordinates(&(x, y): &(i32, i32)){
        println!("Location: ({x}, {y})");
    }



    // Note: patterns are either "refutable" or "irrefutable".
    let a = 5; // irrefutable pattern: can never fail
    let x = Some(1);
    if let Some(x) = x { } // refutable: can fail to match, but there's a condition handling that



    // Match expressions can match multiple patterns
    let x = 1;
    match x {
        // Match against a literal
        1 => println!("one"),
        // Match multiple patterns
        2|3 => println!("some"),
        // Match ranges of values
        4..=9 => println!("multiple"),
        // Match a named variable: irrefutable pattern, always matches. Therefore, comes last.
        x => println!("many"),
    }


    // Match a range of characters
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }



    // Destructuring structs
    let p = Point{x: 0, y: 7};
    let Point{x, y} = p;  // destructure
    println!("Point: x={x}, y={y}");

    // Match zero points
    match p {
        Point{x, y: 0} => println!("On the x axis"),
        Point{x: 0, y} => println!("On the y axis"),
        Point{x, y} => println!("Elsewhere"),
    }



    // Destructuring enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },  // struct-like enum
        Write(String), // tuple-like enum
        ChangeColor(Color),  // nested enum
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        // Match an enum without any data
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // Match a struct-like enum
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        // Match a tuple-like enum
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // Match a nested enum
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }



    // Ignoring parts of a value
    let point = (0, 7);
    if let (0, _) = point {  // ignore one value
        println!("Point on axis");
    }

    let numbers = (2, 4, 8, 16, 32);
    if let (first, .., last) = numbers { // ignore a range
        println!("Numbers ranging from {first} to {last}");
    }





    // A "match guard" is an additional "if" condition in a match arm:
    let num = Some(4);
    match num {
        // Only matches if the condition matches.
        // NOTE: the compiler won't check conditions for exhaustiveness
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }



    // Use `if` to switch branches on and off:
    let x = 4;
    let enabled = false;

    match x {
        (4 | 5 | 6) if enabled => println!("yes"),
        _ => println!("no"),
    }





    // "@ bindings": create a variable while matching
    enum Say {
        Hello{id: i32},
    }

    let msg = Say::Hello{ id: 5 };

    match msg {
        // Lets you capture the value while matching the range
        Say::Hello{ id: id_var @ 3..=7 } => println!("Hello small number {id_var}!"),
        Say::Hello{ id: id_var @ 7..=9 } => println!("Hello big number {id_var}!"),
        Say::Hello{ id } => println!("Hello number {id}!"),
    }
}





struct Point {
    x: i32,
    y: i32,
}
```





# rust/a20_advanced/src


# rust/a20_advanced/src/main.rs

```rust
use std::slice;


fn main() {
    // === Unsafe Rust === //
    // Rust offers memory safety guarantees.
    // But there's a second language hidden inside it that doesn't enforce these rules: "unsafe Rust".
    // Unsafe Rust exist because by nature, static analysis is conservative: it will better reject
    // some valid programs than accept some invalid programs.
    // In these cases, you can use unsafe code to tell the compiler: "trust me, I know what I'm doing".

    // Five unsafe superpowers:
    // 1. Dereferencing a raw pointer
    // 2. Calling an unsafe function/method
    // 3. Creating a safe abstraction over unsafe code
    // 4. Using `extern` functions to call external code
    // 5. Accessing a mutable static variable
    // 6. Implementing an unsafe trait
    // 7. Accessing fields of a union



    // === Dereferencing a raw pointer
    // Unsafe Rust has "raw pointers": they are similar to references.
    // They can be:
    // * immutable: `*const T`
    // * mutable: `*mut T`
    // Raw pointers:
    // * are allowed to ignore the borrowing rules,
    // * aren't guaranteed to point to valid memory,
    // * are allowed to be null
    // * don't implement any automatic clean-up

    let mut num = 5;

    // Raw pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r = 0x012345 as *const i32;  // points somewhere :shrug:

    // You can only dereference them in "unsafe" code
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r);  // don't. just don't. don't even think about it. no.
    }



    // === Calling an unsafe function/method
    // "unsafe" indicates that the function has requirements we need to uphold when we call it.
    unsafe fn dangerous(){}

    unsafe {
        dangerous()
    }



    // === Creating a safe abstraction over unsafe code
    // Let's implement `.split_as_mut()`: a function that takes a mutable slice and splits it in two at a certain index.
    // This function cannot be implemented using safe Rust.
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        assert!(mid <= values.len());

        // This would fail, because we return two mutable references to a single value.
        // Borrowing non-overlapping parts of a slice is okay, but Rust isn't smart enough here:
        // it only sees that we're borrowing from the same slice twice.
        (&mut values[..mid], &mut values[mid..]);  // Error: error[E0499]: cannot borrow `*values` as mutable more than once at a time

        // This implementation works:
        let len = values.len();
        let ptr = values.as_mut_ptr();  // get a raw pointer to the slice

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),  // creates a slice from a raw pointer
                slice::from_raw_parts_mut(ptr.add(mid), len-mid),
            )
        }
    }



    // === Using `extern` functions to call external code
    // To run code written in another language, Rust has `extern` that uses FFI (foreign function interface).
    // An FFI allows a different programming language to use the functions you've defined.

    // Here's how you use "abs" function from the "C" standard library.

    // The "C" which ABI (application binary interface) the external function uses: "C" is most common.
    // It governs how functions are called, in terms of which registers to use, etc.
    extern "C" {
        // List of function signatures
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("abs(-3)={}", abs(-3));
    }

    // You can also define a function for another language to call:
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function!");
    }




    // === Mutable static variables
    // "Global variables": Rust does support them, but they can be problematic with the ownership rules:
    // because if two threads are accessing the same mutable global variable, it can cause a data race.

    // Static variable.
    // They can only store references with the 'static lifetime. It's annotated implicitly.
    // It is different from constants because it have a fixed address in memory, whereas
    // constants are allowed to duplicate their data whenever they're used.
    static HELLO_WORLD: &str = "Hello, world!"; // define outside of `main()`
    // Accessing an immutable static variable is safe.
    println!("My name is {}", HELLO_WORLD);

    // Accessing a mutable static variables is unsafe.
    static mut COUNTER: u32 = 0; // define outside of `main()`
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }




    // === Implementing an unsafe trait
    // A trait is "unsafe" when at least one of its methods has some invariant that the compiler can't verify.
    // By using `unsafe`, we're promising that we'll uphold the invariants that the compiler can't verify.
    unsafe trait Foo {}
    unsafe impl Foo for i32 {}




    // === Accessing fields of a union
    // Used to interface with unions in C code.
    // See "unions" in the Rust reference: https://doc.rust-lang.org/reference/items/unions.html









    // ==== Advanced Traits ==== //

    // === Associated types
    // "Associated types" connect a type placeholder with a trait. It's like a generic.
    // The implementor of the trait will specify the concrete type to be used instead.
    pub trait Iterator {
        type Item; // the placeholder

        fn next(&mut self) -> Option<Self::Item>; // refers to the associated type
    }

    struct Counter{}
    impl Iterator for Counter {
        type Item = u32; // choose the type we implement the trait for

        fn next(&mut self) -> Option<Self::Item> {
            None // Not implemented
        }
    }


    // Question: associated types look very similar to generics. Why not define our iterator like this?
    pub trait GenericIterator<T> {
        fn next(&mut self) -> Option<T>;
    }
    // Because with generics, you can implement a trait multiple times:
    // an `Iterator<string> for Counter`, then another `Iterator<i32> for Counter`.
    // So, when a trait has a generic parameter, it can be implemented for a type multiple times,
    // and we'd need to provide a type annotation every time we called `.next()`.
    // With associated types, we don't need to annotate types: it can only be implemented once.

    // However, generic parameters may have a default value: a "default type parameter".
    // You'll use them in cases where most users won't need to customize your code.
    pub trait GenericIterator2<T=Self> {
        fn next(&mut self) -> Option<T>;
    }

    impl GenericIterator2 for Counter {
        fn next(&mut self) -> Option<Self> {
            None // Not implemented
        }
    }



    // === Operator overloading
    // "Operator overloading": lets you overload the operators by implementing traits from `std::ops`.

    // Example: overload the `Point` `+` operator:
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;  // associated type of a trait: the type returned by `add()`

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        // Added, works :)
        Point { x: 3, y: 3 }
    );


    // The `Add` trait has a default type parameter, which defaults to `Self`.
    // But you can implement `Add<T>` for other types:
    struct Millimeters(u32);
    struct Meters(u32);

    // Implement `+` for Millimeters+Meters
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }





    // === Disambiguation: two traits with the same method name
    trait Pilot { fn fly(&self); }
    trait Wizard { fn fly(&self); }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) { println!("This is your captain speaking."); }
    }
    impl Wizard for Human {
        fn fly(&self) { println!("Up!"); }
    }
    impl Human {
        fn fly(&self) { println!("*waving arms furiously*"); } }

    // Two traits implement a method named "fly". Also, a `Human` has their own.
    // Which one to use?

    let person = Human;
    person.fly();  //-> Human.fly(): the one implemented directly

    // Use fully qualified syntax
    Pilot::fly(&person);  //-> Pilot.fly()
    Wizard::fly(&person); //-> Wizard.fly()

    // Use type casting
    (&person as &dyn Pilot).fly(); //-> Pilot.fly()






    // === Supertraits
    // A "supertrait" requires one trait's functionality within another trait.
    // That is, a trait depends on another trait, and a type must implement both.
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string(); // A method of `Display`
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("{}", output);
            println!("{}", "*".repeat(len + 4));
        }
    }




    // === The Newtype pattern
    // The "newtype" pattern implements external traits on external types.
    // Normally, we're only allowed to implement a trait on a type if either the trait or the type
    // are local to our crate. The "newtype pattern" gets around this restriction by creating a new type.

    // Say, we want to implement `Display` on `Vec<T>`.
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // The downside is that `Wrapper` is a new type, so it doesn't have the methods of the value it's holding :(
    // If we wanted the new type to have every method of the inner type, implement the `Deref` trait
    // on the wrapper and have it return the inner type.

    // Newtypes are useful to make sure the user cannot accidentally provide "Meters" instead of "Millimeters".
    // Newtypes can expose a public API that is different from the API of the private inner type.
    // It can also be used to hide internal implementation.









    // === Advanced Types === //

    // === Type Aliases

    // "Type alias": give an existing type another name.
    // Unlike newtype, we don't get the type checking benefits: you can mix up the types.
    type Kilometers = i32;

    // Values of `Kilometers` will be treated as regular `i32`:
    let distance = (5 as Kilometers) + 3;  //-> type: `i32`

    // Type aliases are used to reduce repetition:
    type Thunk = Box<dyn Fn() + Send + 'static>; // "thunk" is code to be evaluated at a later time, e.g. a closure

    // Type aliases can have generic params
    type Result<T> = std::result::Result<T, std::io::Error>;



    // === The `Never` type
    // `!` -- the "empty type" or "never type": for functions that never return.
    // such functions are called "diverging functions".
    fn bar() -> ! {
        loop {}
    }

    // It is also used in case a `match` returns a `continue`: a no-value:
    loop {
        let guess = "32";

        // The value of `guess` is either `u32`, or `!`: a no-return.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // type `!`.
            // Internally, Rust allows `!` to be coerced into any other type, so `match` is u32
            Err(_) => continue,
            // Also type `!`: never returns
            _ => panic!(),
        };
        break;
    } // `loop` without a `break` also has type `!` :)




    // === Dynamically-sized types and the `Sized` trait
    // "dynamically sized types" (DST) or "unsized types" let us write code using values whose size
    // we can know only at runtime. Rust needs to know how much memory to allocate for any value of
    // a particular type, and all values of a type must use the same amount of memory. But if it was so,
    // all strings would need to have the same length. So in the end, you cannot create a `str` variable:
    let s1: str = "Hello there!"; // Error: the size for values of type `str` cannot be known at compilation time

    // This is why we use `&str` instead: a pointer has a known size: a memory address + length.
    // The rule of thumb: always put dynamically-sized types behind a pointer.
    let s1: &str = "Hello there!";

    // Every trait is a dynamically-sized type: to use traits as trait objects, we must put them
    // behind a pointer: `&dyn Trait` or `Box<dyn Trait>` or ...

    // To work with DSTs, Rust provides the `Sized` trait to determine whether a type's size
    // is known at compile time.

    // Rust implicitly adds a bound on `Sized` to every generic function:
    fn generic<T>(t: T){} // implied: `T: Sized`

    // By default, generic functions will work only on types that have a known size at compile time.
    // However, you can use the following special syntax to relax this restriction:
    fn generic2<T: ?Sized>(t: &T) {
        // now, trait `T` may or may not be `Sized`.
        // Note that `t` is not a `&T`: because the type might not be sized, we need to use it behind some pointer.
    }









    // === Advanced Functions & Closures === //
    // A pointer to a function.
    // Function pointers implement all three of the closure traits: `Fn`, `FnMut`, `FnOnce`.
    type f = fn(i32) -> i32;
    fn with_callback(f: f){}  // pass a function to a function


    // Return a closure from a function
    // Note: got to use a pointer, because the size of this `Fn` closure is unknown
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // If you return an `fn` type instead of a trait, you don't need a pointer:
    fn returns_fn() -> fn(i32) -> i32 {
        |x| x + 1
    }








    // === Macros === //
    // "Macros" refer to:
    // * Declarative macros with `macro_rules!()` syntax
    // * Custom `#[derive]` macros that specify code
    // * Attribute-like macros that define custom attributes
    // * Function-like macros that look like function calls, but operate on tokens

    // Macros: a way of writing code that writes other code. This is "metaprogramming".
    // E.g. the `#[derive]` attribute generates an implementation of various traits for you.
    // E.g. `println!()`: it expands to produce more code than the code you've written.

    // Functions have a fixed number of parameters; macros can take a variable number of parameters.
    // Macros are expanded before the compiler interprets the meaning of the code.

    // === Declarative macros
    // "Declarative macros": allow you to write something similar to the `match` expression:
    // they compare literal Rust source code to a pattern, and replaces it with code associated with the pattern.

    // The `vec!` macro:
    #[macro_export]  // make it available whenever this crate is brought into scope
    macro_rules! vec {  // macro "vec"
        // pattern to match: parentheses, arguments.
        // `$` declares a variable
        // `$x` captures any expression
        // `,` indicates that a literal comma separator character could optionally appear after the code
        // `*` means zero or more matches
        ( $( $x:expr ),* ) => {
            {  // generate a Rust expression block
                let mut temp_vec = vec::new();
                $(  // expand to N `push()` calls
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
        // ... more arms may go here to provide other ways to match
    }

    // === Procedural Macros
    // It acts more like a function: accept some code as an input, operate on that code,
    // and produce some code as an output. No pattern matching.
    // The three kinds of procedural macros:
    // * custom derive
    // * attribute-like
    // * function-like

    // When creating procedural macros, the definitions must reside in their own crate with a special crate type.
    // This is for complex technical reasons that we, the Rust team, hope to eliminate in the future.

    use proc_macro;

    // Define a procedural macro, where `some_attribute` is a placeholder for using a specific macro variety.
    // The function that defines a procedural macro takes a `TokenStream`, and produces a `TokenStream`.
    // This is basically a Rust function that handles Rust AST to generate code.
    // See: https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    }



    // More on macros:
    // https://doc.rust-lang.org/rust-by-example/macros.html
}

```





# rust/a21_appendix/src


# rust/a21_appendix/src/main.rs

```rust
fn main() {
    // Use a keyword where it's not allowed: in a function name
    // This function is called `match`
    fn r#match(needle: &str, haystack: &str) -> bool {
        haystack.contains(needle)
    }
    assert!(r#match("foo", "foobar"));


    // Operator: compound type constraint: all of
    trait A {}
    type Something = dyn A + Send + 'static;


    // Ranges.
    // Right-exclusive range literal:
    let _ = 1..10;
    // Right-inclusive range literal:
    let _ = 1..=10;



    // Numerical literal of specific type
    let _ = 1234u32;
    let _ = 1234usize;

    // Raw string literal: escape characters not processed
    let _ = r"this is a newline sequence: \n";
    let _ = r#"this is a newline sequence: \n"#;
    let _ = r##"this is a newline sequence: \n"##;  // heredoc-like

    // Byte string literal:
    let _ = b"bytes"; // an array of bytes
    let _ = br"bytes";
    let _ = br#"bytes"#;
    let _ = br##"bytes"##;


    // Array literals
    let _ = [1; 32]; // array literal containing 32x copies of `1`

    // Turbofish:  `method::<T>` specifies parameters for a generic function/method in an expression
    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    let _even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<_>>(); // turbofish type





    // #[derive] attribute: applies to a struct/enum, generates code that will implement a trait.
    // Example:
    // * `Display`: how to format the value for end-users?
    // * `Debug`: display the value for developers. Formatted with `println!("{:?}")`
    // * `PartialEq` allows you to compare instances with `==` and `!=`
    //   Two structs are equal if *all* fields are equal.
    // * `Eq` has no methods. It signals that a value is equal to itself.
    //   The value must also be `PartialEq`. Example: `NaN` is not equal to itself.
    // * `PartialOrd` implements `partial_cmp()` for `<`, `<=`, `>=`, `>` operators.
    //   The function returns a `Option<Ordering>`, which is None for `NaN` values that don't compare.
    //   For structs, fields are compared in the order of definition.
    //   For enums, earlier variants compare less.
    // * `Ord` returns `Ordering`, not `Option<Ordering>`: for types that always have a valid ordering.
    // * `Clone` allows a deep copy. It calls `clone()` on every part of the type.
    // * `Copy` allows a shallow copy: only the bits stored on the stack.
    //   Programmers will assume that "clone" is slow, but "copy" is fast.
    // * `Hash` allows to map a value to a fixed-sized hash. Required for `HashMap<K, V>`.
    // * `Default` allows to create a default for the type.
    //   The implementation calls `.default()` on each part of the type.
    //   Is required when you want to use `.unwrap_or_default()`, or use `..Default::default()` for struct update
}

```





# rust/a22_rust_by_example/src


# rust/a22_rust_by_example/src/main.rs

```rust
use std::mem;
use std::fmt;
use std::str;
use std::str::FromStr;
use std::num::ParseIntError;
use std::error;


fn main() {
    // Write formatted text to string
    let _ = format!("value: {}", 32);

    // Print to the console
    print!("value: {}\n", 32);
    println!("value: {}", 32);

    // Print to stderr:
    eprint!("error: {}\n", "error");
    eprintln!("error: {}", "error");

    // Print with index
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Print with named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Convert to hex.
    // Numeric literal with underscore for readability.
    println!("hex: {:X}", 69_420);  //-> 10F2C

    // Zero-pad
    println!("{number:0>5}", number=1); //-> 00001
    println!("{number:0>width$}", number=1, width=5); //-> 00001


    // Array of a specific length
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // all initialized to the same value

    // Arrays are stack allocated (!)
    println!("Array occupies {} bytes", mem::size_of_val(&xs)); // "sizeof"

    // A slice
    let _ = &ys[1 .. 4];

    // Safe access
    match xs.get(99) {
        Some(x) => println!("x={x}"),
        None => println!("nonexistent"),
    }

    // Regular enums start with `0`
    enum Number {
        Zero,
        One,
        Two,
    }
    println!("zero is {}", Number::Zero as i32);  //-> 0

    // C-like enums with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("roses are #{:06x}", Color::Red as i32);  //-> #ff0000



    // === Conversion. `From` and `Into`.
    // The `From` and `Into` traits are inherently linked: if you can convert A -> B, it should be easy B -> A

    // The `From` trait's `.from()` method
    let _my_string = String::from("123");

    // Example: caset from `i32`:
    struct Num(i32);
    impl From<i32> for Num {
        fn from(item: i32) -> Self {
            Num(item)
        }
    }

    let _n = Num::from(30);

    // The `Into` is simply the reciprocal of the `From` trait.
    // If you have implemented the `From` for your type, `Into` will call it when necessary.
    // It's sort of implemented automatically.
    struct Numb(i32);
    impl Into<Numb> for i32 {
        fn into(self) -> Numb {
            Numb(self)
        }
    }
    let num: Numb = 5.into();


    // === Conversion. `TryFrom` and `TryInto`
    // Generic traits for converting between types, but are used for fallible conversions.
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        // Converts only if a condition works
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));


    // === `ToString` and `FromString`
    // To convert a type to a `String`, implement `ToString`.
    // But rather, implement the `fmt::Display`!

    // For parsing, use the `parse()` function.
    // Either arrange for type inference, or specify the type to parse using the turbofish syntax:

    let parsed: i32 = "5".parse().unwrap(); // type inference
    let parsed = "10".parse::<i32>().unwrap(); // turbofish






    // === Expressions

    // `if..else` is an expression
    let value = if parsed > 0 {
        parsed
    } else {
        -parsed
    };

    // `loop` is an expression.
    // Typical use case: retry an operation until it succeeds
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter >= 10 {
            break counter*2;
        }
    }; // returns 20
    println!("result={result}"); //-> 20


    // let-else.
    // Set value to a variable with a match of a refutable pattern.
    let s = "54";
    let Ok(n) = u64::from_str(s) else {
        // Diverge: break, return panic
        return
    };

    // while let.
    // Makes awkward match sequences more tolerable.
    let list = vec![1, 2, 3];
    let mut it = list.iter();
    while let Some(i) = it.next() {
        println!("i={i}");
    }





    // === Use configuration
    // The cfg!() macro is a boolean expression that checks config:

    // This function only gets compiled if the target OS is linux
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }

    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }


    // It can be used to control error flow: abort or unwind?
    // Control it with:
    // $ rustc lemonade.rs -C panic=abort
    #[cfg(panic = "unwind")]
    fn ah(){ println!("we failed!"); }

    #[cfg(panic="abort")]
    fn ah(){ panic!("Run!"); }

    if cfg!(panic="abort") {
        panic!("aborted");
    }






    // === Option<> and unwrap
    struct Person {
        job: Option<Job>,
    }
    struct Job {
        phone_number: Option<String>,
    }

    fn get_phone_number(person: Option<Person>) -> Option<String> {
        // The `?` operator will terminate and return `None` from the function
        person?.job?.phone_number
    }

    // Use `Option::map()` for simple Some=>Some and None=>None mappings:
    fn decorated_phone_number(phone_number: Option<String>) -> Option<String> {
        // Maps Some() values, passthru for `None`
        phone_number.map(|num| String::from("+7") + &num.to_owned())
    }

    // `.and_then()`: often used to chain fallible operations.
    // While `.map()` always returns a value for `Some()`, this method can return `None`
    let arr = vec![1, 2, 3];
    assert_eq!(arr.get(2).and_then(|x| Some(x+10)), Some(13));

    // `.or()` is chainable and eagerly evaluates its argument
    // `.or()` receives an Option<T> and picks the first non-empty one
    assert_eq!(arr.get(2).or(arr.get(3)).unwrap(), &3);

    // `.or_else()` is chainable and evaluates lazily
    assert_eq!(arr.get(2).or_else(|| arr.get(3)).unwrap(), &3);

    // `.get_or_insert()` evaluates an `Option<>` to make sure it contains a value
    // `.get_or_insert_with()` evaluates lazity
    assert_eq!(arr.get(2).get_or_insert(&-1), &mut &3);
    assert_eq!(arr.get(2).get_or_insert_with(|| &-1), &mut &3);




    // === Result<T, E>
    // `Result` is a richer version of the `Option<T>`: it describes the possible error.
    let _ = "42".parse::<i32>().unwrap();

    // The main() func can have a `Result`:
    fn main() -> Result<(), ParseIntError> {
        Ok(())
    }

    // Result also has `.map()`, `.and_then()`, etc
    let v = "42".parse::<i32>().and_then(|x| Ok(x*10));  //-> Result<i32, ParseIntError>
    match v {
        Ok(n) => println!("v={n}"),
        // Check for a specific error type
        Err(ParseIntError{..}) => panic!("Parse failed"),
    }





    // === Define an error type

    // Define an error type
    #[derive(Debug, Clone)]
    struct NoFirstElement;

    impl fmt::Display for NoFirstElement {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "no first element")
        }
    }


    // To use multiple errors in a function, use an Enum:
    #[derive(Debug, Clone)]
    enum ParseFirstItemError {
        NoFirstElement,
        ParseError,
    }
    impl fmt::Display for ParseFirstItemError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "parsing failed")
        }
    }

    // Use it: a func with a result, and two documented errors
    fn parse_first_item(vec: Vec<&str>) -> Result<i32, ParseFirstItemError> {
        vec.first()
            // Change the error
            .ok_or(ParseFirstItemError::NoFirstElement)
            .and_then(|s| {
                s.parse::<i32>()
                    // change the error too
                    .map_err(|_| ParseFirstItemError::ParseError)
            })
    }

    // A way to write simple code is to `Box<Error>`.
    // The drawback is the error type is only known at runtime.
    fn something() -> Result<i32, Box<dyn error::Error>> {
        "zzz".parse::<i32>()
            .map_err(|e| e.into())  // map error
    }


    // In functions, use `?` operator to return upon error.
    // A `?` is almost exactly the same as an `unwrap()` that `return`ed instead of panicking.
    // In addition, it converts the error to the target type: using the `From` trait.

    impl error::Error for NoFirstElement {} // allows From conversion
    impl error::Error for ParseFirstItemError {} // allows From conversion

    fn parse_second_item(vec: Vec<&str>) -> Result<i32, Box::<dyn error::Error>> {
        let item = vec.first().ok_or(NoFirstElement)?;  // `?` returns error
        let parsed = item.parse::<i32>()?; // `?` returns error
        Ok(2 * parsed)
    }



    // An alternative to Boxing errors (`Box<dyn Error>`) is wrapping them:
    enum DoubleFirstItemError {
        EmptyVec,
        Parse(ParseIntError),
    }

    impl From<ParseIntError> for DoubleFirstItemError {
        fn from(err: ParseIntError) -> Self {
            DoubleFirstItemError::Parse(err)
        }
    }

    fn double_first_item(vec: Vec<&str>) -> Result<i32, DoubleFirstItemError> {
        let first = vec.first().ok_or(DoubleFirstItemError::EmptyVec)?;
        let parsed = first.parse::<i32>()?; // `From` implicitly converts it! Trait `From<ParseIntError>` is implemented for `DoubleError`
        Ok(parsed * 2)
    }









    // === Collections: vector, string, bytestring
    // All values in Rust are stack-allocated by default.
    // Values can be "boxed" (allocated on the heap) by creating a `Box<T>`.
    struct Point(u32, u32);
    let point = Box::new(Point(0, 0));
    println!("Size: {}", mem::size_of_val(&point));  //-> 8

    // Vector: re-sizable array
    let counts: Vec<i32> = (0..10).collect();

    // String: vector of bytes, but guaranteed to always be a valid UTF-8.
    // String is heap-allocated, growable, and not null-terminated!
    let pangram: &str = "the quick brown fox jumps over the lazy dog";

    // Because strings keep their length in memory, we can iterate over words -- without allocations!
    for word in pangram.split_whitespace().rev() {
        println!("Word: {word}");
    }

    // Create a new, growable string
    let mut string = String::new();
    string.push_str("Hello");
    string.push('!');

    // The trimmed string is a slice to the original string, so no new allocation is performed
    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat"); // allocates new memory

    // Array of bytes that's mostly text, but contains non-UTF8 sequences
    let bytestring: &[u8; 21] = b"this is a byte string";
    let escaped = b"\x52\x75\x73\x74 as bytes";

    // Converting a byte string to String can fail
    if let Ok(my_str) = str::from_utf8(bytestring) {
        println!("And the same as text: '{}'", my_str);
    }






    // === Path
    // `Path` represents file paths.
    // The prelude automatically exports either `posix::Path` and `windows::Path`.
    // `Path` is immutable. Owned mutable version is `PathBuf`.
    // Note that `Path` is not internally an UTF-8 string: it's an `OsString`.
    use std::path::Path;
    let path = Path::new(".");
    let mut new_path = path.join("a").join("b");
    new_path.push("c");
    new_path.push("myfile.tar.gz");
    let path_str = match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => s,
    };
    println!("Path: {path_str}");




    // === Files
    use std::fs::File;
    use std::io::prelude::*;

    // Create a path to the file
    let path = Path::new("hello.txt");

    {
        // Create a file: write-only mode, overwrite.
        let mut file = match File::create(&path){
            Err(why) => panic!("cannot create {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // Write to the file
        const contents: &str = "Hello world!";
        match file.write_all(contents.as_bytes()){
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => (),
        }
    } // auto-close

    // Open the file for reading
    let mut file = match File::open(&path){
        Err(why) => panic!("cannot open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // Read into a string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }
    println!("contents: {s:?}");


    // Read lines from a file
    use std::fs::read_to_string;
    let lines: Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();

    // Read lines from a file, line by line, using `BufReader` to reduce intermediate allocations:
    use std::io::{self, BufRead};
    let lines = File::open(path).and_then(|f| Ok(io::BufReader::new(f).lines()));

    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(line) = line {
                println!("Line: {}", line);
            }
        }
    }


    // === Run command
    use std::process::Command;

    // Execute a command
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap();

    if output.status.success() {
        // Get output, ignore UTF-8 errors
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        // Get error output
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

}

```





# rust
# Further reading

* [Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)
* [Macros](https://doc.rust-lang.org/rust-by-example/macros.html)
* [Async Rust](https://rust-lang.github.io/async-book/)
* [Rust Language Reference](https://doc.rust-lang.org/reference/index.html)
* [Rust Standard Library](https://doc.rust-lang.org/std/index.html)
* [Rustonomicon: Unsafe Rust](https://doc.rust-lang.org/nomicon/index.html)





# embedded

# Embedded Rust

Reading:

* [Embeeded Discovery Book](https://docs.rust-embedded.org/discovery/): small fun projects to teach you bare metal programming.
* [The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/): if you are familiar with embedded development
* [OS Tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials): learn to write an embedded OS in Rust! On Pi.
* [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/): a deep dive into the implementation of the foundational crates: linker, symbols, and ABIs.

More resources:

* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust): curated list of libraries and teaching materials







# Introduction


## Libstd
The standard library contains primitives to interact with OS systems: FS, network, memory, threads, etc.
In a bare-metal environment, no code has been loaded before your program, so there's no OS abstractions
and no POSIX that the standard library depends upon.

To prevent Rust from loading the standard library, use `#![no_std]`. It's a crate-level attribute

The missing libstd also provides a runtime: takes care of setting up stack overflow protection,
processes command-line arguments, spawns the main thread before a program's `main` is invoked, etc.
This runtime also won't be available.

The platform-agnostic parts of the standard library are available through [libcore](https://doc.rust-lang.org/core/).
It also excludes things that are not always desirable in an embedded environment, like the memory allocator:
use crates of your choice.

The `libcore` contains:
APIs for language primitives (floats, strings, slices, etc),
APIs that expose processor features like "atomic" operations and SIMD instructions,
etc.

However, it lacks APIs for anything that involves platform integration:
because it can be used for any kind of bootstrapping (stage 0) code like bootloaders, firmware, or kernels.



## Tooling

Install:

* `rustup`: installs Rust and tooling
* [`cargo-generate`](https://github.com/cargo-generate/cargo-generate): a cargo subcommand to generate projects from templates. Alternatively, clone a git repo.
* `cargo-binutils`: tools for LLVM use to inspect binaries: `objdump`, `nm`, `size`
* `qemu-system-arm`: emulate ARM systems locally, run programs without having any hardware with you!
* GDB: you may not always have the luxury to log stuff to the host console.
  Also, LLDB doesn't yet support `load` that uploads the program to the target hardware.
  So, currently, GDB is recommended.
* OpenOCD/ESPtool: GDB isn't able to communicate directly with the hardware: it needs a translator.
  OpenOCD translates between GDB protocol and ST-Link's USB protocol. It knows to to read/write flash.
  It also knows how to interact with ARM CoreSight debug peripheral,
  which interacts with memory-mapped registers allow to breakpoint/watchpoint, read CPU registers, continue, etc.

Also you might want to add:

* `cargo-embed`: cargo-embed is the big brother of `cargo-flash`.
  It can flash a target, and it can also open an RTT terminal as well as a GDB server.
  Installed as a part of `probe-rs` tools.
* `minicom` to open a terminal with a USB-connected device

Install:

```console
$ sudo apt install cargo-binutils qemu-system-arm gdb-multiarch libudev-dev
$ cargo install cargo-generate
$ cargo install probe-rs --features cli,ftdi
$ sudo apt install esptool espflash stm32flash openocd
```

However, the recommended way is to install using Rustup:
this is a more mature environment:

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install cargo-binutils cargo-generate
$ cargo install probe-rs --features cli,ftdi
$ rustup component add llvm-tools-preview
```

To use ST-Link without root privileges, you may need to create a udev rule:
[more info here](https://doc.rust-lang.org/stable/embedded-book/intro/install/linux.html):

```udev
# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"

# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess"

# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```




Test that it works: with ST-Link: one of these:

```console
$ openocd -f interface/stlink.cfg -f target/stm32f3x.cfg
$ openocd -f interface/stlink-v2.cfg -f target/stm32f3x.cfg
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
...
Info : Target voltage: 2.919881
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

if you don't have the "breakpoints" line, use a different config file.







## Enable Target Cross-Compilation

By default, Rust only supports native compilation.

Check the list of targets:

```console
$ rustc --print target-list
thumbv4t-none-eabi
thumbv5te-none-eabi
thumbv6m-none-eabi
thumbv7a-pc-windows-msvc
thumbv7a-uwp-windows-msvc
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv7neon-unknown-linux-musleabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
...
riscv32imc-esp-espidf
...
```

This is how architectures are added:

```console
$ rustup target add thumbv6m-none-eabi
```




## Terminology

* PAC: Peripheral Access Crate. Provides a safe-ish direct interface to the peripherals of the chip.
  Normally you only deal with PACs if the higher level doesn't fulfil your needs.
* HAL: Hardware Abstraction Layer. It builds upon the chip's PAC and provides an abstraction.
  You can use the chip without knowing all the special behavior of the chip.
* BSP: Board Support Crate. Abstracts a whole board away at once, with all its sensors, leds, etc.
  Quite often, you will work with the HAL, and get drivers for your sensors from crates.io.

The central piece: (`embedded-hal`)[https://crates.io/crates/embedded-hal]: provides a set of traits
that describe behavior common to specific peripherals. These are common interfaces.
Drivers that are written in such a way are called platform agnostic. Most drivers are.






## Flashing and Debugging

NOTE: for ESP32 on Xtensa, see `./a01-rust-on-esp32`.
This tutorial is for `cargo-embed`: it supports: ARM, RISC-V, CMSIS-DAP, STLink, JLink. See [probe-rs](https://probe.rs/).

Create a new project:

```console
$ cargo new <projectname>
```

Microcontroller programs are different from standard programs in two aspects:

* `#![no_std]`: this program won't use the `std` crate, which assumes an underlying OS.
  It will instead use the `core` create: a subset of `std` that can run on bare metal systems.
* `#![no_main]`: this program won't use the standard `main` interface, which is
  tailored for command-line applications that receive arguments.
  Instead, we'll use the `#[entry]` attribute from the `cortex-m-rt` create to define
  a custom entry point.

Check this `src/main.rs` file:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {  // `!`: this program never returns
  // Set up RTT structures in the memory and read/write data from/to them
  // Note: you can set up multiple channels: input channels, output channels. No problem.
  rtt_init_print!();

  let _y;
  let x = 42;
  _y = x;

  // infinite loop; just so we don't leave this stack frame
  loop {
    // Print RTT
    rprintln!("Hello, world!");
  }
}
```

Notice the `.cargo/config` file: it tweaks the linking process to tailor the memory layout
of the program to the requirements of the target device:

```
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

The `Embed.toml` or `.embed.toml` file: it configures `cargo-embed`:

```toml
# "default" on the outer level is the configuration profile name.
# Use "cargo embed --config <profile>" to use a different one.
[default.general]
# The chip we're working with
chip = "STM32F401CCUx"

[default.reset]
# Halt the chip after we flashed it
halt_afterwards = true

# RTT: Real time transfers. It's a mechanism for transferring data between the host and the device.
# Supports multiple channels (ringbuffers)
[default.rtt]
enabled = true

[default.gdb]
# Start a GDB server after flashing?
enabled = true
```

To cross-compile, pass `rustc --target`. It's simple.
But before compiling, download a pre-compiled version of the standard library for your target:

```console
$ rustup target list
$ rustup target add ...
```

Build the binary for your target:

```console
$ cargo build --target thumbv7em-none-eabihf
```

Check the file:

```console
$ cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-headers
```

Run with RTT enabled, see the terminal:

```console
$ cargo embed
```

Now, "flashing" is the moving of our program into the microcontroller's memory.





# embedded/a01-rust-on-esp32
# Rust on ESP32

This is for the ESP32 board.

Reading:

* [ESP-RS book](https://esp-rs.github.io/book/)
* [ESP-RS training](https://esp-rs.github.io/std-training/): writing apps with `std`
* [Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust): a collection

In the [esp-rs](https://github.com/esp-rs/) organization:

* `esp-*` repositories are focused on `no_std` applications: e.g. `esp-hal`
* `esp-idf-*` are focused on `std` apps: e.g. `esp-idf-hal`

## `std` vs `no_std`

### `std` on ESP32

Unlike most other embedded platforms, Espressif supports the Rust standard library.
Most notably, this means you'll have arbitrary-sized collections like `Vec` or `HashMap` at your disposal, as well as generic heap storage using `Box`.

You're also free to spawn new threads, and use synchronization primitives like `Arc` and `Mutex` to safely share data between them.
Still, memory is a scarce resource on embedded systems, and so you need to take care not to run out of it - threads in particular can become rather expensive.

Espressif provides a C-based development framework: [ESP-IDF](https://github.com/espressif/esp-idf), which provides a [newlib](https://sourceware.org/newlib/) environment that has enough functionality to build the Rust `std` on top of it.

When using `std`, you have access to a lot of `ESP-IDF` features: threads, mutexes, collections, random numbers, sockets, etc.

Services like Wi-Fi, HTTP client/server, MQTT, OTA updates, logging etc. are exposed via Espressif's open source IoT Development Framework, ESP-IDF.
It is mostly written in C and as such is exposed to Rust in the canonical split crate style:

* the [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys) crate provides the actual `unsafe` bindings to the IDF development framework that implements access to drivers, Wi-Fi, and more
* the higher-level [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) crate implements safe and comfortable Rust abstractions: it implements abstractions from [embedded-svc](https://github.com/esp-rs/embedded-svc): wi-fi, network, httpd, logging, etc
* [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal): implements traits from `embedded-hal` and other traits using the `esp-idf` framework: analog/digital conversion, digital I/O pins, SPI communication, etc.

You might want to use the `std` when your app:
* requires rich functionality (network, file I/O, sockets, complex data structures)
* for portability: because the `std` crate provide APIs that can be used across different platforms
* rapid development


### `no_std` on ESP32

Using `no_std` may be more familiar to embedded Rust developers: it uses a subset of `std`: the `core` library.

See crates:

* [esp-hal](https://github.com/esp-rs/esp-hal):	Hardware abstraction layer
* [esp-pacs](https://github.com/esp-rs/esp-pacs):	Peripheral access crates
* [esp-wifi](https://github.com/esp-rs/esp-wifi):	Wi-Fi, BLE and [ESP-NOW](https://www.espressif.com/en/solutions/low-power-solutions/esp-now) support
* [esp-alloc](https://github.com/esp-rs/esp-alloc):	Simple heap allocator
* [esp-println](https://github.com/esp-rs/esp-println):	`print!`, `println!`
* [esp-backtrace](https://github.com/esp-rs/esp-backtrace):	Exception and panic handlers
* [esp-storage](https://github.com/esp-rs/esp-storage):	Embedded-storage traits to access unencrypted flash memory

You might want to use the `no_std` when:

* You need a small memory footprint
* Direct hardware control. Because `std` adds abstractions that make it harder to interact directly with the hardware
* Real-time constraints or time-critical applications: because `std` can introduce unpredictable delays and overhead
* Custom requirements: fine-grained control over the behavior of an application


## Preparation

Espressif SoCs are based on two different architectures: RISC-V and Xtensa.

* Modern chips: ESP32-C/H/P are based on RISC-V.
* Older chips: ESP32, ESP32-S use Tensilica Xtensa.

For ESP32-C2, C2, C6, H2, P4:
[Tools for RISC-V Targets only](https://esp-rs.github.io/book/installation/riscv.html).

For all chips (Xtensa *and* RISC-V):

* install `espup`: simplifies installing and maintaining the components required to build
* Run `espup install` to install the toolchain: Rust fork, nightly toolchain, LLVM fork, GCC toolchain

Instead of installing, you can use a Docker image: [espressif/idf-rust
](https://hub.docker.com/r/espressif/idf-rust/tags):

```console
$ docker pull espressif/idf-rust:all_latest
```

But if you still want to install:
first of all, it is recommended to use `rustup` rather than your distro's package manager!
Your `rustup` will be able to determine which toolchain to use: see [rustup overrides](https://rust-lang.github.io/rustup/overrides.html).

But anyway:

```console
$ sudo apt install llvm-dev libclang-dev clang libuv-dev
$ cargo install cargo-espflash espflash ldproxy
$ espup install
```

Now source this file in every terminal:

```console
$ . $HOME/export-esp.sh
```

or use direnv's `.envrc`:

```bash
#!/bin/bash

# direnv:
# will automatically configure your environment
# see:
# * https://direnv.net/
# * https://github.com/direnv/direnv/wiki
# * https://github.com/direnv/direnv/blob/master/stdlib.sh

watch_file ~/export-esp.sh
. ~/export-esp.sh
```

Also, you'd need to set the toolchain for this folder:

```console
$ rustup override set esp
```

## Start a project

Generate a project:

```console
$ cargo install cargo-generate
$ cargo generate esp-rs/esp-template
```

Questions from `cargo generate`:

* Which MCU? `esp32` with Xtensa architecture

Now build and flash:

```console
$ cargo build
$ cargo run
```

You can use `cargo run` because of `.cargo/config.toml`, which configures the build target and the runner:

```toml
[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor"
```



## VSCode

Rust analyzer can behave strangely without `std`.
Add this to `settings.json`:

```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```




# embedded/a01-rust-on-esp32/.envrc

```
PATH_add /home/kolypto/.rustup/toolchains/esp/bin

watch_file ~/export-esp.sh
. ~/export-esp.sh


```





# embedded/a01-rust-on-esp32/.cargo


# embedded/a01-rust-on-esp32/.cargo/config.toml

```toml
[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor"


[build]
rustflags = [
  "-C", "link-arg=-Tlinkall.x",

  "-C", "link-arg=-nostartfiles",
]
target = "xtensa-esp32-none-elf"

[unstable]
build-std = ["core"]

```





# embedded/a01-rust-on-esp32/src


# embedded/a01-rust-on-esp32/src/main.rs

```rust
// Don't use the standard library.
// Don't use main(), which is tailored for command-line applications.
// Instead, we'll use the #[entry] attribute from the `xtensa-lx-rt` crate
#![no_std]
#![no_main]

// Installs a panic handler.
// You can use other crates, but `esp-backtrace` prints the address which can be decoded to file/line
use esp_backtrace as _;

// Provides a `println!` implementation
use esp_println::println;

// Bring some types from `esp-hal`
use hal::{peripherals::Peripherals, prelude::*};
use hal::{clock::ClockControl, Delay, timer::TimerGroup, Rtc};
use hal::{IO};

// The entry point.
// It must be a "diverging function": i.e. have the `!` return type.
#[entry]
fn main() -> ! {
    // HAL drivers usually take ownership of peripherals accessed via the PAC
    // Here we take all the peripherals from the PAC to pass them to the HAL drivers later.
    // This is singleton design pattern: one owner, here:
    let peripherals = Peripherals::take();

    // Sometimes a peripheral is coarse-grained and doesn't exactly fit the HAL drivers.
    // Here we split the SYSTEM peripheral into smaller pieces which get passed to drivers.
    // `split()` creates a `hal::system::SystemParts` HAL structure from a PAC structure:
    // we are sort of "splitting" a PAC structure into HAL structures: peripherals, registers, etc.
    let mut system: hal::system::SystemParts = peripherals.DPORT.split();

    // Configure the CPU clock: use the max() frequency. Then apply (freeze)
    // let clocks = ClockControl::boot_defaults(system.clock_control).freeze(); // defaults
    // let clocks = ClockControl::max(system.clock_control).freeze(); // max speed
    let clocks = match "max" {
        // Choose a frequency
        "max" => ClockControl::max,
        _ => ClockControl::boot_defaults,
    }(system.clock_control).freeze();



    // Disable watchdogs.
    // The ESP32 chip has three watchdog timers "wdt": one in each of the two timer modules, and one in the RTC module.
    // Only the RDWT (RTC Watchdog) can trigger the "system reset" (reset the entire chip)
    // The two other timers can do: "interrupt", "CPU reset", "core reset".
    // They have four stages each, each with a timeout and action.
    // Let's disable them all.

    // Disable the RWDT (RTC watchdog).
    // "RTC" is a real-time clock. It can wake the device up from a low power mode.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    rtc.rwdt.disable();

    // Disable TIMG watchdog timers ("MWDT": main watchdog timers).
    // If not, the SoC would reboot after some time.
    // Another way is to feed the watchdog.
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut system.peripheral_clock_control);
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut system.peripheral_clock_control);
    let mut wdt0 = timer_group0.wdt;
    let mut wdt1 = timer_group1.wdt;
    wdt0.disable();
    wdt1.disable();


    // App config file.
    // Values are read from "cfg.toml" during build.
    let app_config = CONFIG;



    // IO handle: it enables us to create handles for individual pins.
    // GPIOs need to be configured before use:
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);



    // Set GPIO2 as an output: this is the LED.
    // Configure it to be a PUSH-PULL output: i.e. low=GND (push down), high=VCC (pull up).
    // Set its initial state: HIGH
    let mut led = io.pins.gpio2.into_push_pull_output();
    led.set_high().unwrap();

    // Set GPIO36 as input
    let button = io.pins.gpio36.into_pull_up_input();



    // Init a timer
    // The MCU has several timers. They can do things for us: e.g. pause the execution for some time.
    let mut timer0 = timer_group0.timer0;

    // Or use this lame `Delay` which is a busy-wait timer: loop + count cycles
    let mut delay = Delay::new(&clocks);

    loop {
        println!("Loop...");
        // Blink
        // led.toggle().unwrap();

        // If the button is pressed (or the pin is touched) — blink
        if button.is_high().unwrap() {
            for _ in 0..10 {
                led.set_high().unwrap();
                delay.delay_ms(50_u32);

                led.set_low().unwrap();
                delay.delay_ms(50_u32);
            }
        }

        // Sleep a bit
        delay.delay_ms(app_config.loop_timer);

        // Sleep using a timer
        let mut del_var = app_config.loop_timer.millis();
        timer0.start(del_var);
        while timer0.wait() != Ok(()) {}
    }
}


/// This configuration is picked up at compile time from `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    //#[default("")]
    //wifi_ssid: &'static str,

    #[default(500u32)]
    loop_timer: u32,
}

```





# embedded/a02-rust-on-esp32-std
# Rust on ESP32 with `std`

Generate a project:

```console
$ cargo generate esp-rs/esp-idf-template cargo
$ cd <project-folder>
$ rustup override set esp
```

* choose "STD support = yes"
* when choosing the IDF version, note that not all chips support it.

Recommended: set the toolchain directory to "global":
otherwise, each new project will have its own instance of the toolchain and eat up disk space:

*   Add this to `.cargo/config.toml`:

    ```toml
    [env]
    ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
    ```

*   Add this to `rust-toolchain.toml`:

    ```toml
    [toolchain]
    channel = "nightly-2023-02-28" # change this line
    ```

We'll use `toml_cfg`. All add `anyhow` to be used in the build script:

```console
$ cargo add toml_cfg anyhow esp-idf-hal embedded-svc
$ cargo add esp-idf-sys --features=binstart
$ cargo add --build toml_cfg anyhow
```




Further reading for ESP32 `std` programming:

* [HTTPS Server](https://esp-rs.github.io/std-training/03_4_http_server.html) and [code](https://github.com/esp-rs/std-training/tree/main/intro/http-server)
* [MQTT Client](https://esp-rs.github.io/std-training/03_5_0_mqtt.html) and [code](https://github.com/esp-rs/std-training/tree/main/intro/mqtt)
* [I2C, I/O, Sensors, Interrupts](https://esp-rs.github.io/std-training/04_0_advanced_workshop.html)

Further reading for ESP32:

* [Awesome ESP32](https://github.com/esp-rs/awesome-esp-rust)



# embedded/a02-rust-on-esp32-std/cfg.toml

```toml
[a02-rust-on-esp32-std]
wifi_ssid = "RT-WiFi-2FCB"
wifi_psk = "Euief5xaRm"

```



# embedded/a02-rust-on-esp32-std/build.rs

```rust
// Note: in `build.rs` we don't need to explicitly import crates.
// These "[build-dependencies]" are added automatically

fn main() -> anyhow::Result<()> {
    // Prints build args
    embuild::espidf::sysenv::output();

    // Check & import `cfg.toml`
    if !std::path::Path::new("cfg.toml").exists() {
        anyhow::bail!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` as a template.");
    }
    let app_config = CONFIG; // const `CONFIG` is auto-generateed
    if app_config.wifi_ssid == "" || app_config.wifi_psk == "" {
        anyhow::bail!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    }

    // // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641 :
    // // > "rustc-link-arg does not propagate transitively"
    // // But build actually fails if we enable these
    // embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    // embuild::build::LinkArgs::output_propagated("ESP_IDF")

    // Return
    Ok(())
}

// App config: the WiFi network to connect to.
// The config is taken from `cfg.toml` and imported into Rust as a value
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

```





# embedded/a02-rust-on-esp32-std/src


# embedded/a02-rust-on-esp32-std/src/main.rs

```rust
use esp_idf_hal::{
    prelude::Peripherals,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};
use anyhow::{bail, Result};
use core::str;
use log;

use a02_rust_on_esp32_std::{wifi, httpclient}; // our lib

fn main() -> Result<()> {
    // Need to call this once: applies patches to the runtime.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Let HAL take the peripherals
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // Print something
    println!("Started :)");

    // Connect to WiFi
    let _wifi = match wifi::connect(
        CONFIG.wifi_ssid,
        CONFIG.wifi_psk,
        peripherals.modem,
        sysloop,
    ){
        Ok(wifi) => {
            log::info!("Connected to Wi-Fi network {:?}!", CONFIG.wifi_ssid);
            wifi
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    // HTTP request
    httpclient::get_url("https://api.myip.com/")?;

    Ok(())
}


// App config. Auto-generated as `CONFIG`
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

```



# embedded/a02-rust-on-esp32-std/src/lib.rs

```rust
// Define the modules' structure
mod lib {
    // Need `pub` -- because otherwise `pub use` won't be able to re-publish them.
    pub mod wifi;
    pub mod httpclient;
}

// Re-publish
pub use crate::lib::{wifi, httpclient};

```





# embedded/a02-rust-on-esp32-std/src/lib


# embedded/a02-rust-on-esp32-std/src/lib/wifi.rs

```rust
use anyhow::{bail, Result};
use embedded_svc::wifi::{
    AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration,
};
use esp_idf_hal::peripheral;
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::BlockingWifi, wifi::EspWifi};
use log::info;


/// Connect to a WiFi network
pub fn connect(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
    // Auth method
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    // Init Wi-Fi
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;
    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    // Start Wi-Fi
    info!("Starting wifi...");
    wifi.start()?;

    // Scan networks
    info!("Scanning...");
    let ap_infos = wifi.scan()?;
    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    // Choose a channel
    let channel = if let Some(ours) = ours {
        info!("Found configured access point {} on channel {}", ssid, ours.channel);
        Some(ours.channel)
    } else {
        info!("Configured access point {} not found during scanning, will go with unknown channel", ssid);
        None
    };

    // Configure the WiFi adapter
    wifi.set_configuration(&Configuration::Mixed(
        // "Mixed" is a Client + Access Point. Yes, we can be an access point.
        ClientConfiguration {
            ssid: ssid.into(),
            password: pass.into(),
            channel,
            auth_method,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    // Connect to the AP Access Point
    info!("Connecting wifi...");
    wifi.connect()?;

    // Get an IP address (DHCP)
    info!("Waiting for DHCP lease...");
    wifi.wait_netif_up()?;

    // Get IP info and print it
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi DHCP info: {:?}", ip_info);

    // Done
    Ok(Box::new(esp_wifi))
}
```



# embedded/a02-rust-on-esp32-std/src/lib/httpclient.rs

```rust
use anyhow::{bail, Result};
use core::str;
use esp_idf_sys;
use embedded_svc::{
    http::{client::Client, Status},
    io::Read,
};
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
};


/// Download data from an HTTP URL
// The `AsRef<str>` means the function accepts anything that implements the trait: both `&str` and `String`
pub fn get_url(url: impl AsRef<str>) -> Result<()> {
    // Create a client: EspHttpConnection, then Client
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // Open a GET request
    let request = client.get(url.as_ref())?;

    // Sed the request
    let response = request.submit()?;
    let status = response.status();
    println!("Response code: {}\n", status);

    // Check the HTTP code
    match status {
        200..=299 => {
            // Read response, buffer size = 256
            let mut buf = [0_u8; 256];
            let mut offset = 0; // offset in the buffer
            let mut total = 0;  // total number of bytes read
            let mut reader = response;

            // Keep reading
            loop {
                // Read into the buffer, starting at `offset`
                if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                    // Read nothing? stop reading.
                    if size == 0 {
                        break;
                    }
                    total += size;

                    // Try converting the bytes into UTF-8 and print
                    let size_plus_offset = size + offset;
                    match str::from_utf8(&buf[..size_plus_offset]) {
                        Ok(text) => {
                            // Print
                            print!("{}", text);
                            // Empty the buffer
                            offset = 0;
                        },
                        Err(error) => {
                            let valid_up_to = error.valid_up_to();
                            unsafe {
                                print!("{}", str::from_utf8_unchecked(&buf[..valid_up_to]));
                            }

                            // Move bytes in the buffer
                            buf.copy_within(valid_up_to.., 0);
                            offset = size_plus_offset - valid_up_to;
                        }
                    }
                }
            }
            println!("Total: {} bytes", total);
        }
        _ => bail!("Unexpected response code: {}", status),
    }

    Ok(())
}

```





# embedded/a03-esp32-serial
## Serial port

"Serial communication" is asynchronous (without a clock signal): where two devices exchange data serially,
one bit at a time, using two data lines + a common ground.

Both parties must agree on how fast data will be sent.
The common configuration is: 1 start bit, 8 bits of data, 1 stop bit, baud rate of 115200 bps.

Today's computers don't have serial ports, but our SoC has a USB-to-serial converter:
it exposes a serial interface to the microcontroller, and a USB interface to the computer, which will see the microcontroller as a virtual serial device.

The computer will see it as a TTY device: `/dev/ttyUSB0` or `/dev/ttyACM0`.
You can send out data by simply writing to this file:

```console
$ echo 'Hello, world!' > /dev/ttyACM0
```

Here's how to open a terminal:

```console
$ screen -U /dev/ttyUSB0 115200
C-a \

$ minicom --device /dev/ttyUSB0 -b 115200
C-a x

$ picocom /dev/ttyUSB0 --b 115200
C-a C-x
```

## UART

The microcontroller has a peripheral called UART: Universal Async Receiver/Transmitter.
It can be configured to work with several communication protocols: e.g. serial.
We'll use it to talk to your computer.






# embedded/a03-esp32-serial/src


# embedded/a03-esp32-serial/src/main.rs

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    prelude::{*, nb::block},
    peripherals::Peripherals,
    peripherals::Interrupt,
    clock::ClockControl, Delay,
    IO, Uart,
    interrupt, uart,
};
use esp_backtrace as _;
// use debouncr::{debounce_3, Edge};  // debounce button press
use core::fmt::Write;  // writeln!() here
use embedded_io;  // read(&buf)

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system: hal::system::SystemParts = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    // Logging
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");

    // Get hold of the LED and the button
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    let mut button = io.pins.gpio0.into_pull_up_input();

    // Get hold of UART.
    // With ESP32, it can be mapped to any GPIO pins, but we use TX/RX pins connected to USB:
    let mut uart0 = Uart::new_with_config(
        peripherals.UART0,
        Some(hal::uart::config::Config{
            baudrate: 115_200,  // the speed
            ..Default::default()
        }),
        // The pings to use: Tx and Rx
        Some(hal::uart::TxRxPins::new_tx_rx(
            // our board has a CP2102 USB-UART converter connected to U0TXD (GPIO1)  and U0RXD (GPIO3) pins.
            // Let's use them: then all our output to this `uart0` will end up in /dev/ttyUSB0 :)
            io.pins.gpio1.into_push_pull_output(),
            io.pins.gpio3.into_floating_input(),
        )),
        &clocks,
        &mut system.peripheral_clock_control,
    );
    // .. or do the same thing, with defaults:
    // let mut uart0 = Uart::new(peripherals.UART0, &mut system.peripheral_clock_control); // with defaults


    // Example: write to UART while the button is pressed
    if false {
        loop {
            // Toggle the LED while the button is pressed
            if !button.is_input_high() {
                led.toggle().unwrap();

                // Speak into UART
                writeln!(uart0, "Button Press!\n").unwrap();
            }

            // Sleep
            delay.delay_ms(50u32);
        }
    }

    // Example: echo server. Reading from UART
    // Heapless: static data structures that don't require dynamic memory allocation
    // All heapless data structures store their memory allocation inline:
    use heapless::Vec;

    // Allocate a buffer: can store up to 128 bytes
    let mut buf: Vec<u8, 128> = Vec::new();

    loop {
        buf.clear();

        // TODO: how to read the whole string?
        // let n =embedded_io::Read::read(&mut uart0, &mut buf).unwrap();

        loop {
            // Read bytes into the buffer
            let byte = nb::block!(uart0.read()).unwrap();
            if buf.push(byte).is_err() {
                write!(uart0, "error: buffer full\n").unwrap();
                break;
            }

            // Enter. Done.
            if byte == 13 {  // <enter>
                // Reverse the string and print it
                for byte in buf.iter().rev().chain(&[b'\n']) {
                    nb::block!(uart0.write(*byte)).unwrap();
                }
                break;
            }
        }

        nb::block!(uart0.flush()).unwrap()
    }
}

```





# embedded/a04-i2c-display
# I2C

"I2C": Inter-Integrated Circuit: synchronous serial communication protocol.

Uses two lines: a data line (SDA) and a clock line (SCL).
Because a clock line is used to synchronize the communication, this is a *synchronous* protocol.


The "controller" is the device that starts and drives the communication.

Several devices, both controllers and targets, can be connected to the same bus.

A controller communicates with a device by first broadcasting its address to the bus.
This address can be 7bits or 10bits long.

No other device can use the bus until the controller stops the communication (!)

The clock determines how fast data can be exchanged:
usually 100 kHz (standard mode) or 400 kHz (fast mode)

Protocol:

1. Controller sends START
2. Controller broadcasts target address (7 or 10 bits) + 1 R/W bit. It's set to "WRITE" for "controller -> target" communication, and "READ" for "controller <- target" communication.
3. Target responds with ACK
4. repeat ( Send one byte + Respond with ACK )
5. Controller broadcasts STOP (or RESTART and go back to 2)

## ADXL345 Accelerometer

What I have here is an ADXL345 accelerometer with I2C, SPI, and two configurable "interrupt" pins to report tap/double-tap/falling.

It is configurable: has a number of writable registers that contain the settings. The registers also contain the readings.

In a sense, these sensors are very similar to the peripherals inside the microcontroller.
The difference is that their registers are not mapped into the microcontroller's memory:
instead, their registers have to be acessed via the I2C bus.

Some accelerometer modules also have a magnetometer: LSM303AGR , MPU-6050.

### Practice

First: find out the target address of the accelerometer.
With ADXL345, the address is `0x1D`, followed by the R/W bit: this translates to `0x3B` (R) and `0x3A` (W).

Second: lots of I2C chips will have some sort of device identification register.
With this device ID register, we can verify that we are indeed talking to the device we expect:
in our case, `DEVID` (`0x00` register) contains `11100101`.

## SSD1780 Display

This is a 128x64 OLED display with I2C and SPI interfaces.

Its address is `011110` + `1/0`.






# embedded/a04-i2c-display/src


# embedded/a04-i2c-display/src/main.rs

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    prelude::*,
    clock::ClockControl, peripherals::Peripherals,
    Delay,
    i2c::I2C, IO,
};
use core::fmt::Write;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{
    prelude::*,
    I2CDisplayInterface, Ssd1306,
    mode::BufferedGraphicsMode,  mode::TerminalMode,
};



#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let mut io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new I2C peripheral
    let mut i2c = I2C::new(
        peripherals.I2C0, // we have 2 I2C peripherals
        io.pins.gpio21,  // SDA pin to use
        io.pins.gpio22,  // SCL pin to use
        100_u32.kHz(),  // frequency. 100kHz is the "standard mode"
        &mut system.peripheral_clock_control,
        &clocks,
    );


    // let interface = I2CDisplayInterface::new(i2c);
    // let mut display = Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate0,
    // ).into_buffered_graphics_mode();
    // display.init().unwrap();

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_6X10)
    //     .text_color(BinaryColor::On)
    //     .build();

    // Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // display.flush().unwrap();


    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate0,
    // ).into_terminal_mode();
    // display.init().unwrap();
    // display.clear().unwrap();

    // // Spam some characters to the display
    // for c in 97..123 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }
    // for c in 65..91 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }


    loop {
        // Get us a buffer and read into it
        let mut data = [0u8; 22];
        // i2c.write_read(<device_addr>, <register>, <buffer>)
        // i2c.write_read(DISPLAY_ADDR, &[0x00], &mut data).ok();
        println!("{:?}", data);

        delay.delay_ms(500_u32);
    }
}

const DISPLAY_ADDR: u8 = 0b111100;
const ACCELEROMETER_ADDR: u8 = 0x3A;

```





# embedded
# Further reading on embedded Rust

* [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/)
* For ESP32: [esp32-hal](https://crates.io/crates/esp32-hal) and [esp32 (PAC)](https://crates.io/crates/esp32)
* For ESP32: [awesome-esp](https://github.com/esp-rs/awesome-esp-rust) and [awesome-esp (non-Rust)](https://github.com/agucova/awesome-esp)
* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust): curated list of libraries and teaching materials





