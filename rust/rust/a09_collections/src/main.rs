#[allow(unused_variables)]
fn main() {
    // === Vector === //
    // Vector: Vec<T> is a list of items of the same type, stored in the heap, next to each other in memory.
    let v: Vec<i32> = Vec::new();

    // Empty vector needs a type.
    // When values are provided, the compiler will infer the type from the values:
    let mut v = vec![1, 2, 3]; // macro

    // Add elements
    // NOTE: don't forget to make the value mutable!
    v.push(4);

    // Get element
    let third: &i32 = &v[2];  // Will panic if index is out of bounds
    let third: Option<&i32> = v.get(2); // Will not panic, but return an Option<T>
    match third {
        Some(third) => println!("Third={third}"),
        None => println!("Third=<none>"),
    }

    // Iterate
    for i in &mut v { // iterate mutable: make changes
        *i += 50;
    }
    for i in &v { // iterate immutable: just read
        println!("i={i}");
    }

    // Vectors can store values that are the same type.
    // Let's use enums to store different value types:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];






    // === String === //
    // `str` literals are stored in the program's binary.
    // `String` are growable, mutable, owned, UTF8-encoded.
    // Strings are implemented as a vector of bytes + methods used when those bytes are interpreted as text.
    let s = "initial string".to_string();
    let s = String::from("initial_string");

    // Append a string
    let mut s = String::new();
    s.push_str("Hello");

    // Append a byte, a single character
    s.push('!');

    // Concat strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // NOTE: `s1` has been moved here, `s1` can no longer be used
                        // NOTE: `&s2` because of the method signature: `fn add(self, s: &str)`

    // Format: add multiple strings together
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Indexing. Because of UTF-8, you can only use ranges:
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // takes the first 4 bytes: 2 letters. It will panic if a letter is broken.

    // Iterate over characters
    for c in "Зд".chars() { // iterate: characters
        println!("{c}");
    }
    for b in "Зд".bytes() { // iterate: bytes
        println!("{b}");
    }






    // === Hash Maps === //
    // HashMap.
    // Uses `SipHash` by default: it's slow, but resistant to DoS attacks with hash tables.
    // Use a faster "hasher" if you like: a type that implements the `BuildHasher` trait.
    use std::collections::HashMap;

    // Create a hash map
    let mut scores = HashMap::new();

    // Add values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Lookup a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get()       -> Option<&i32>
    // .copied()    -> Option<i32>
    // .unwrap_or() -> i32

    // Iterate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // If a value is already there: 1) overwrite
    scores.insert(String::from("Blue"), 25);

    // If a value is already there: 2) insert only if not present
    scores.entry(String::from("Blue")).or_insert(50);

    // If a value is already there: 3) update
    let text = "hello world wonderful world";
    let mut wordcount = HashMap::new();
    for word in text.split_whitespace() {
        let count = wordcount.entry(word).or_insert(0);
        *count += 1;
    }
}
