// Use our sub-crate as a library
use adder;

fn main() {
    // Use our library
    let x = adder::add_one(1);
    println!("Using a library: {x}");
}
