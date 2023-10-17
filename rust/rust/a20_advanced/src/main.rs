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
        // println!("r is: {}", *r);  // don't. just don't.
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
        // (&mut values[..mid], &mut values[mid..]);  // Error: error[E0499]: cannot borrow `*values` as mutable more than once at a time

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









    // === Advanced Types === //

}
