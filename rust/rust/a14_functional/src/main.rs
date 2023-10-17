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