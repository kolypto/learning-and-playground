// This is the binary crate: src/main.src
// It will be compiled into a binary.


// Tells the compiler to include garden.rs
// It tells the compiler: this module exists and should be compiled as a part of this crate.
mod garden;

// Use module: garden/vegetables.rs, struct Asparagus
// Tells the compiler: give me a shortcut to this thing.
use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
