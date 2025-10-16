fn main() {
    // Use a keyword where it's not allowed: in a function name
    // This function is called `match`
    fn r#match(needle: &str, haystack: &str) -> bool {
        haystack.contains(needle)
    }
    assert!(r#match("foo", "foobar"));


    // Operator: compound type constraint: all of
    trait A {}
    type Something = dyn A + Send + 'static;


    // Ranges.
    // Right-exclusive range literal:
    let _ = 1..10;
    // Right-inclusive range literal:
    let _ = 1..=10;



    // Numerical literal of specific type
    let _ = 1234u32;
    let _ = 1234usize;

    // Raw string literal: escape characters not processed
    let _ = r"this is a newline sequence: \n";
    let _ = r#"this is a newline sequence: \n"#;
    let _ = r##"this is a newline sequence: \n"##;  // heredoc-like

    // Byte string literal:
    let _ = b"bytes"; // an array of bytes
    let _ = br"bytes";
    let _ = br#"bytes"#;
    let _ = br##"bytes"##;


    // Array literals
    let _ = [1; 32]; // array literal containing 32x copies of `1`

    // Turbofish:
    // `method::<T>` specifies parameters for a generic function/method in an expression
    // Append `::<T>` to a function call
    // Called this way because ::<> resembles a fish :D
    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    let _even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<_>>(); // turbofish type





    // #[derive] attribute: applies to a struct/enum, generates code that will implement a trait.
    // Example:
    // * `Display`: how to format the value for end-users?
    // * `Debug`: display the value for developers. Formatted with `println!("{:?}")`
    // * `PartialEq` allows you to compare instances with `==` and `!=`
    //   Two structs are equal if *all* fields are equal.
    // * `Eq` has no methods. It signals that a value is equal to itself.
    //   The value must also be `PartialEq`. Example: `NaN` is not equal to itself.
    // * `PartialOrd` implements `partial_cmp()` for `<`, `<=`, `>=`, `>` operators.
    //   The function returns a `Option<Ordering>`, which is None for `NaN` values that don't compare.
    //   For structs, fields are compared in the order of definition.
    //   For enums, earlier variants compare less.
    // * `Ord` returns `Ordering`, not `Option<Ordering>`: for types that always have a valid ordering.
    // * `Clone` allows a deep copy. It calls `clone()` on every part of the type.
    // * `Copy` allows a shallow copy: only the bits stored on the stack.
    //   Programmers will assume that "clone" is slow, but "copy" is fast.
    // * `Hash` allows to map a value to a fixed-sized hash. Required for `HashMap<K, V>`.
    // * `Default` allows to create a default for the type.
    //   The implementation calls `.default()` on each part of the type.
    //   Is required when you want to use `.unwrap_or_default()`, or use `..Default::default()` for struct update
}
