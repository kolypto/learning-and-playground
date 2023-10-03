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