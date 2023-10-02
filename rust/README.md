# Go





# rust/a01-hello/src


# rust/a01-hello/src/main.rs

```rust
// Create a new app with:
// $ cargo new a01-hello

// Run me with:
// $ cargo run
// or
// $ rustc main.rs
// $ ./main

// To build it:
// For release, with optimizations:
// $ cargo build --release

// Dependency added with:
// $ cargo add ferris-says
use ferris_says::say; // use function `say()` from the crate
use std::io::{stdout, BufWriter};

fn main() {
    // Simple print.
    // This is a Rust macro: as indicated by the `!`
    println!("Hello, world!");
    
    // Print using Ferris-Says
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let message_length = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, message_length, &mut writer).unwrap();
}

```





# rust/a01-hello


# rust/a01-hello/Cargo.toml

```toml
[package]
name = "a01-hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
ferris-says = "0.3.1"

```

