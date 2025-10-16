use std::fmt::Display;
use std::fmt::Debug;


#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    //=== Generics ===//

    // A function that finds the largest item in the slice
    // The <T> must be a comparable value: have a *trait* that allows comparisons:
    //            std::cmp::PartialOrd
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0]; // bad but fine for now

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

    // Constraints: we can implement methods only on Point<f32> instances
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

    // Implement a trait on a struct:
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // No duck typing in Rust: a struct must explicitly implement a trait to satisfy it.
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }


    // A trait function can have a body: this is the default behavior.
    // It will be overridden by specific implementation.
    pub trait Summary2 {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    impl Summary2 for NewsArticle {
        // use the default implementation
    }


    // A function that has a trait as a parameter:
    // it will accept any object that has Summary implemented.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // `&impl Summary` parameter is actually syntax sugar for a longer form:
    // the "trait bound":
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // Note that the "trait bound" is actually a generic with a constraint:
    // - "impl Trait" is static dispatch (monomorphization):
    //   compiler generates a separate version of func for each concrete type used
    // - "dyn Trait" is dynamic dispatch (vtable) at runtime:
    //   single function uses *vtable* for method lookup at runtime

    // Specify multiple trait bounds
    pub fn notify3(item: &(impl Summary + Display)) {}
    pub fn notify4<T: Summary + Display>(item: &T) {}

    // Trait bounds can get very long. That's why Rust has an alternate syntax:
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where // table of trait bounds:
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
    impl<T: Display> std::string::ToString for T {
        // --snip--
    }






    // === Lifetimes === //
    // Lifetime: a kind of generic. It ensures that references are valid as long as we need them to be.
    //
    // Every reference has a "lifetime": the scope for which that reference is valid.
    // Most of the time, lifetimes are implicit or inferred.
    // We only have to annotate lifetimes when the lifetimes of references could be related in a few different ways.

    // The main aim of lifetimes is to prevent "dangling references".

    // This variable has no value,
    // but Rust won't let you use this variable unless you initialize it.
    let r;

    // `x` only lives inside this scope. So it we wanted to do `r = &x`, it would fail:
    //   `x` doesn't live long enough.
    // `r` lives longer, though, because it's defined in the outer scope.
    {
        let x = 5;
        r = &x;  // will fail: > `x` does not live long enough
    }
    println!("r: {}", r);


    // This will not fail: because `x` lives long enough.
    let x = 5;
    let r = &x;




    // Let's write a function that returns the longest of two strings.
    // We don't want it to take ownership, so its parameters are references:
    fn longest1(x: &str, y: &str) -> &str {
        if x.len() > y.len() { x } else { y }
    }
    // This won't compile: > error: expected named lifetime parameter
    // because Rust doesn't know whether the return value refers to `a` or `b`.
    // Actually, we don't know either: it's dynamic.
    // So the borrow checker won't be able to evaluate lifetimes.

    // We need to add a lifetime annotation that expresses the following constraint:
    // the returned reference will be valid as long as both the parameters are valid.
    //
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
    // It's just a named marker.
    // They are used to describe the relationships of the lifetimes of multiple references:
    // if two values have the same marker, they're expected to be valid together.



    // So far, our structs had owned types.
    // When a struct holds references, you'd need to add a lifetime annotation on every reference:
    struct ImportantExcerpt<'a> {
        // <'a> means the struct can't outlive the reference in the `part` field.
        // That is, the "part" field must be valid for as long as the struct is valid.
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
    //
    // Rules:
    // 1. Every reference parameter gets a separate lifetime parameter
    // 2. If there is exactly 1 input lifetime parameter, it is assigned to all output lifetime parameters
    // 3. If one parameter is `&self` (it's a method), the lifetime of `self` is assigned to all output lifetimes.
    //
    // "Input lifetimes": lifetimes on function or method parameters
    // "Output lifetimes": lifetimes on return values




    // The 'static lifetime:
    // this reference can live for the entire duration of the program.
    // All string literals are static.
    let s: &'static str = "I have a static lifetime";






    // All generics together, in one function:
    // traits, trait bounds, lifetimes

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
