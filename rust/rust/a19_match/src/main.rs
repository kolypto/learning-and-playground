fn main() {
    // "Patterns" are for matching against the structure of types:
    // * Literals
    // * Destructured arrays
    // * Variables
    // * Wildcards
    // * Placeholders
    // Patterns describe the *shape* of data.

    let x = Some(1);

    // Match expression.
    // A match expression must be "exhaustive": i.e. all possibilities must be accounted for
    let _y = match x {
        // Match arms
        None => 0,
        Some(i) => i + 1,

        // Default match-all:
        _ => 0,
    };


    // `if let` expressions: can match only one value to an expression
    let favorite_color: Option<&str> = None;

    if let Some(color) = favorite_color {
        println!("Your favorite color: {color}");
    }


    // `while let` conditional loop: run a loop for as long as the pattern continues to match
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Top: {top}");
    }



    // `for` loops can match a pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("#{index}: {value}");
    }




    // `let` can match a pattern
    let (x, y, z) = (1, 2, 3);



    // Function parameters can also be patterns
    fn print_coordinates(&(x, y): &(i32, i32)){
        println!("Location: ({x}, {y})");
    }



    // Note: patterns are either "refutable" or "irrefutable".
    let a = 5; // irrefutable pattern: can never fail
    let x = Some(1);
    if let Some(x) = x { } // refutable: can fail to match, but there's a condition handling that



    // Match expressions can match multiple patterns
    let x = 1;
    match x {
        // Match against a literal
        1 => println!("one"),
        // Match multiple patterns
        2|3 => println!("some"),
        // Match ranges of values
        4..=9 => println!("multiple"),
        // Match a named variable: irrefutable pattern, always matches. Therefore, comes last.
        x => println!("many"),
    }


    // Match a range of characters
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }



    // Destructuring structs
    let p = Point{x: 0, y: 7};
    let Point{x, y} = p;  // destructure
    println!("Point: x={x}, y={y}");

    // Match zero points
    match p {
        Point{x, y: 0} => println!("On the x axis"),
        Point{x: 0, y} => println!("On the y axis"),
        Point{x, y} => println!("Elsewhere"),
    }



    // Destructuring enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },  // struct-like enum
        Write(String), // tuple-like enum
        ChangeColor(Color),  // nested enum
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        // Match an enum without any data
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // Match a struct-like enum
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        // Match a tuple-like enum
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // Match a nested enum
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }



    // Ignoring parts of a value
    let point = (0, 7);
    if let (0, _) = point {  // ignore one value
        println!("Point on axis");
    }

    let numbers = (2, 4, 8, 16, 32);
    if let (first, .., last) = numbers { // ignore a range
        println!("Numbers ranging from {first} to {last}");
    }





    // A "match guard" is an additional "if" condition in a match arm:
    let num = Some(4);
    match num {
        // Only matches if the condition matches.
        // NOTE: the compiler won't check conditions for exhaustiveness
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }



    // Use `if` to switch branches on and off:
    let x = 4;
    let enabled = false;

    match x {
        (4 | 5 | 6) if enabled => println!("yes"),
        _ => println!("no"),
    }





    // "@ bindings": create a variable while matching
    enum Say {
        Hello{id: i32},
    }

    let msg = Say::Hello{ id: 5 };

    match msg {
        // Lets you capture the value while matching the range
        Say::Hello{ id: id_var @ 3..=7 } => println!("Hello small number {id_var}!"),
        Say::Hello{ id: id_var @ 7..=9 } => println!("Hello big number {id_var}!"),
        Say::Hello{ id } => println!("Hello number {id}!"),
    }
}





struct Point {
    x: i32,
    y: i32,
}