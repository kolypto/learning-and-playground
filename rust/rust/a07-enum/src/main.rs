#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // An Enum encodes meaning along with data.
    // It says that a value is one of a possible set of values.
    enum IpAddressKind {
        V4,
        V6,
    }
    let kind = IpAddressKind::V4;

    // We can put data directly into each enum variant: it will have an associated value:
    // Internally, it is implemented as a *tagged union*:
    //   discriminant (tag) + enough space for the largest variant's data
    enum IpAddress {
        V4(String),
        V6(String),
    }

    // Every enum variant is also a function that constructs an instance of the enum:
    let loopback = IpAddress::V4(String::from("127.0.0.1"));

    // Another advantage: enum variants can have different types associated with them:
    enum IpAddress2 {
        // We can use individual bytes.
        // The standard library uses structs.
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress2::V4(127, 0, 0, 1);
    let loopback = IpAddress2::V6(String::from("::1"));

    // More variants with embedded types:
    enum Message {
        // No data
        Quit,
        // named fields, like a struct
        Move { x: i32, y: i32 },
        // a String
        Write(String),
        // three ints
        ChangeColor(i32, i32, i32),
    }

    // We could have defined 4 structs instead, but then it wouldn't be so easy
    // to define a function to take any of these kinds of messages.

    // We can define methods on enums
    impl Message {
        fn call(&self){
            // ...
        }
    }
    Message::Write(String::from("hello")).call();






    // === Option<T>
    // The Option Enum: a value that can be something, or nothing.
    // Rust does not have NULLs: with this enum, you would have to handle both outcomes every time.
    // It's defined like this:
    //   enum Option<T> { // a generic
    //      None,
    //      Some(T),
    //      // `Some` and `None` are actually included in the prelude: you don't need to bring them into scope.
    //   }

    let some_number = Some(5); // Option<i32>
    let absent_number: Option<i32> = None;

    // Here's how to use it
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // ERROR: cant add `i8` and `Option<i8>` because they are different types.
    // You first need to convert `Option<i8>` into `i8` and handle the nullable case explicitly.






    // === Match expression
    // Match expression: it will run different code depending on which variant of the enum it has.
    // Match compares a value against a series of patterns: literal values, variable names, wildcards, ...
    // See more in Chapter 18.

    // Coin matching: map to a value
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // Each arm is an expression that returns a value from `match` expression
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            // If you omit a variant, you will get this error:
            // > pattern `Coin::Quarter` not covered
        }
    }

    // Patterns can also bind values:
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }
    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }

    enum UsCoin {
        Penny,
        Nickel,
        Dime,
        // Until 2008, the US minted quarters with different designs for each of the 50 states on one side.
        // No other coins got state designs, so only quarters have this extra value.
        Quarter(UsState),
    }
    fn uscoin_value_in_cents(coin: UsCoin) -> u8 {
        match coin {
            UsCoin::Penny => 1,
            UsCoin::Nickel => 5,
            UsCoin::Dime => 10,
            // Add a variable for `state` to bind the value
            UsCoin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }






    //=== Matching with Option<T>
    // A function that takes an optional int, and if there's a value, adds 1:
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // Match and bind
            Some(i) => Some(i+1),
        }
    }

    // Matches are exhaustive: the patterns must cover all possibilities.
    // The compiler won't let you forget a variant.



    // Catch-all pattern:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // the pattern is a variable.
        // NOTE: put it last, because patterns are matched in order.
        other => move_player(other),
    }

    // Alternatively, use `_` to ignore the value and return a unit:
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // we don't need the value, and we won't run any code. Return the unit.
        _ => (),
    }






    // === if let
    let config_max = Some(3u8);

    // too wordy: annoying boilerplate:
    // we have to add _ => () after processing just one variant.
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // more concise.
    // It takes a pattern and an expression and does the matching, but for only one arm.
    // Like pattern matching/destructuring, with enum unwrap.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // You can include an `else` with an `if let`
    let coin = UsCoin::Dime;
    if let UsCoin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        //...
    }

    // Here's a way to stay on the happy path and exit early:
    fn describe_state_quarter(coin: UsCoin) -> Option<String> {
        // Take advantage of the fact that expressions produce a value
        let state = if let UsCoin::Quarter(state) = coin {
            state
        } else {
            // Exit early
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }
    // To make this common pattern nicer to express, Rust has `let ... else`:
    fn describe_state_quarter2(coin: UsCoin) -> Option<String> {
        // Unwrap or exit early
        let UsCoin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(_other: i32){}
