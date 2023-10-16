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

/// Doccomments. Start with triple /
///
/// # Examples
/// Here we go with Markdown.
/// This Markdown will be used on crates.io to document the usage of your library.
///
/// ```rust
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




[dependencies]
# Use our sub-library as a dependency :)
adder = { path = "./adder" }

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

