use crate::garden::vegetables::Asparagus;

// Tells the compiler to include garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
