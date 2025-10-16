
// Here's how to use it
fn main() {
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // here, `s` goes out of scope and is dropped. Its memory goes away. You cannot reference it anymore.