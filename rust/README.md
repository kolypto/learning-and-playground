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

