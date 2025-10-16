// This is a library crate.
// The binary crate can use it as if it was an external library.
// See Chapter 12 for an example.

// Define a module.
// A module groups related definitions together.
//
// The code can be in back_office.rs
mod back_office {
    // All items are private by default.
    // So, if you want to make an item private, you put it in a module.
    // That is, Rust is hiding inner implementation details by default.

    // Items in a parent module can't use the private items inside child modules:
    // child modules are *implementation details* of the parent; parents can't see them.
    //
    // However, child modules know the context they're defined in:
    // e.g. they can use public methods and definitions of the parent.
    //
    // Also, if the child has made something `pub`, then the parent could use it.
    //
    // ❗ So, every definition flows inward, but only `pub` definitions flow outward.

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
