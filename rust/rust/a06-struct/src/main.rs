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

    // Define a function that creates an instance of the structure.
    // Here we use the "field init shorthand" to avoid repetition (like in JS)
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

    // dbg!(): print value={value} like in Python
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
    // Methods are defined within the context of a struct, enum, or a trait object.
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
    let rect = Rectangle::square(32);
    println!("Area: {}", rect.area());
    // Automatic de/referencing.
    // Remember how C has two different operators for calling methods: `.` and `->`?
    // Rust does *automatic referencing and dereferencing* here: i.e.
    // when you call `object.somethind()`, Rust automatically adds in `&`, `&mut`, or `*`
    // so that `object` matches the signature of the method.
    // Calling methods is one of the few places in Rust with this behavior.
}
