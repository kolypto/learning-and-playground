fn main() {
    // Function call
    let ret = another_function(42, 'h');
    println!("Return: {ret}");

    // `if` expression.
    // Branches are called "arms".
    // When too many arms, use `match`.
    let number = 3;
    if number < 5 {
        println!("number < 5");
    } else if number > 100 {
        println!("number > 100");
    } else {
        println!("number is somewhere between 5..100");
    }

    // Because `if` is an expression, we can use it in a `let`.
    // See expressions below
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Result: {number}");

    // `loop`: forever
    loop {
        println!("forever");

        break; // okay, not forever
        #[allow(unreachable_code)]
        continue; // skip iteration
    }

    // A `loop` is an expression: it can return a value: useful for retry operations
    let mut counter = 0;
    let result = loop {
        // Keep incrementing
        counter += 1;

        // Stop when reached 10 retries
        if counter == 10 {
            // This value will be returned out of the loop expression
            break counter * 2;
        }
    };
    println!("Counter result: {result}");

    // Loop labels: break out of many levels
    'out: loop {
        loop {
            break 'out;
        }
    }

    // while: conditional loop
    let mut number = 3;
    while number >= 0 {
        number -= 1;
    }

    // for .. in .. : Loop through a collection
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("a[]: {element}");
    }

    // Loop through a range
    // Range: (1..4)
    for number in (1..4).rev() {
        println!("Countdown: {number}!");
    }
}

// Function definition
fn another_function(value: i32, unit: char) -> i32 {
    println!("Another function; value={value}{unit}.");

    // Statement: instructions that do not return a value
    // Expression: evaluate to a resultant value
    let _x = 6; // statement
    // let _x = (let _x = 6); // Fails: "note: variable declaration using `let` is a statement"

    // A block is an expression: it can return a value (final statement)
    let _x = {
        let x = 3;
        x + 1
    };

    // Same applies to functions:
    // return value is the final expression: it returns implicitly
    // Note: an expression is without ";"
    _x
}
