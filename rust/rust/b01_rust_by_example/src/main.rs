use std::mem;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::error;


fn main() {
    // Write formatted text to string
    let _ = format!("value: {}", 32);

    // Print to the console
    print!("value: {}\n", 32);
    println!("value: {}", 32);

    // Print to stderr:
    eprint!("error: {}\n", "error");
    eprintln!("error: {}", "error");

    // Print with index
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Print with named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Convert to hex.
    // Numeric literal with underscore for readability.
    println!("hex: {:X}", 69_420);  //-> 10F2C

    // Zero-pad
    println!("{number:0>5}", number=1); //-> 00001
    println!("{number:0>width$}", number=1, width=5); //-> 00001


    // Array of a specific length
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // all initialized to the same value

    // Arrays are stack allocated (!)
    println!("Array occupies {} bytes", mem::size_of_val(&xs)); // "sizeof"

    // A slice
    let _ = &ys[1 .. 4];

    // Safe access
    match xs.get(99) {
        Some(x) => println!("x={x}"),
        None => println!("nonexistent"),
    }

    // Regular enums start with `0`
    enum Number {
        Zero,
        One,
        Two,
    }
    println!("zero is {}", Number::Zero as i32);  //-> 0

    // C-like enums with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("roses are #{:06x}", Color::Red as i32);  //-> #ff0000



    // === Conversion. `From` and `Into`.
    // The `From` and `Into` traits are inherently linked: if you can convert A -> B, it should be easy B -> A

    // The `From` trait's `.from()` method
    let _my_string = String::from("123");

    // Example: caset from `i32`:
    struct Num(i32);
    impl From<i32> for Num {
        fn from(item: i32) -> Self {
            Num(item)
        }
    }

    let _n = Num::from(30);

    // The `Into` is simply the reciprocal of the `From` trait.
    // If you have implemented the `From` for your type, `Into` will call it when necessary.
    // It's sort of implemented automatically.
    struct Numb(i32);
    impl Into<Numb> for i32 {
        fn into(self) -> Numb {
            Numb(self)
        }
    }
    let num: Numb = 5.into();


    // === Conversion. `TryFrom` and `TryInto`
    // Generic traits for converting between types, but are used for fallible conversions.
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        // Converts only if a condition works
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));


    // === `ToString` and `FromString`
    // To convert a type to a `String`, implement `ToString`.
    // But rather, implement the `fmt::Display`!

    // For parsing, use the `parse()` function.
    // Either arrange for type inference, or specify the type to parse using the turbofish syntax:

    let parsed: i32 = "5".parse().unwrap(); // type inference
    let parsed = "10".parse::<i32>().unwrap(); // turbofish



    // === Expressions

    // `if..else` is an expression
    let value = if parsed > 0 {
        parsed
    } else {
        -parsed
    };

    // `loop` is an expression.
    // Typical use case: retry an operation until it succeeds
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter >= 10 {
            break counter*2;
        }
    }; // returns 20
    println!("result={result}"); //-> 20


    // let-else.
    // Set value to a variable with a match of a refutable pattern.
    let s = "54";
    let Ok(n) = u64::from_str(s) else {
        // Diverge: break, return panic
        return
    };

    // while let.
    // Makes awkward match sequences more tolerable.
    let list = vec![1, 2, 3];
    let mut it = list.iter();
    while let Some(i) = it.next() {
        println!("i={i}");
    }





    // === Use configuration
    // The cfg!() macro is a boolean expression that checks config:

    // This function only gets compiled if the target OS is linux
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }

    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }


    // It can be used to control error flow: abort or unwind?
    // Control it with:
    // $ rustc lemonade.rs -C panic=abort
    #[cfg(panic = "unwind")]
    fn ah(){ println!("we failed!"); }

    #[cfg(panic="abort")]
    fn ah(){ panic!("Run!"); }

    if cfg!(panic="abort") {
        panic!("aborted");
    }






    // === Option<> and unwrap
    struct Person {
        job: Option<Job>,
    }
    struct Job {
        phone_number: Option<String>,
    }

    fn get_phone_number(person: Option<Person>) -> Option<String> {
        // The `?` operator will terminate and return `None` from the function
        person?.job?.phone_number
    }

    // Use `Option::map()` for simple Some=>Some and None=>None mappings:
    fn decorated_phone_number(phone_number: Option<String>) -> Option<String> {
        // Maps Some() values, passthru for `None`
        phone_number.map(|num| String::from("+7") + &num.to_owned())
    }

    // `.and_then()`: often used to chain fallible operations.
    // While `.map()` always returns a value for `Some()`, this method can return `None`
    let arr = vec![1, 2, 3];
    assert_eq!(arr.get(2).and_then(|x| Some(x+10)), Some(13));

    // `.or()` is chainable and eagerly evaluates its argument
    // `.or()` receives an Option<T> and picks the first non-empty one
    assert_eq!(arr.get(2).or(arr.get(3)).unwrap(), &3);

    // `.or_else()` is chainable and evaluates lazily
    assert_eq!(arr.get(2).or_else(|| arr.get(3)).unwrap(), &3);

    // `.get_or_insert()` evaluates an `Option<>` to make sure it contains a value
    // `.get_or_insert_with()` evaluates lazity
    assert_eq!(arr.get(2).get_or_insert(&-1), &mut &3);
    assert_eq!(arr.get(2).get_or_insert_with(|| &-1), &mut &3);




    // === Result<T, E>
    // `Result` is a richer version of the `Option<T>`: it describes the possible error.
    let _ = "42".parse::<i32>().unwrap();

    // The main() func can have a `Result`:
    fn main() -> Result<(), ParseIntError> {
        Ok(())
    }

    // Result also has `.map()`, `.and_then()`, etc
    let v = "42".parse::<i32>().and_then(|x| Ok(x*10));  //-> Result<i32, ParseIntError>
    match v {
        Ok(n) => println!("v={n}"),
        // Check for a specific error type
        Err(ParseIntError{..}) => panic!("Parse failed"),
    }




    // === Define an error type

    // Define an error type
    #[derive(Debug, Clone)]
    struct NoFirstElement;

    impl fmt::Display for NoFirstElement {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "no first element")
        }
    }


    // To use multiple errors in a function, use an Enum:
    #[derive(Debug, Clone)]
    enum ParseFirstItemError {
        NoFirstElement,
        ParseError,
    }
    impl fmt::Display for ParseFirstItemError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "parsing failed")
        }
    }

    // Use it: a func with a result, and two documented errors
    fn parse_first_item(vec: Vec<&str>) -> Result<i32, ParseFirstItemError> {
        vec.first()
            // Change the error
            .ok_or(ParseFirstItemError::NoFirstElement)
            .and_then(|s| {
                s.parse::<i32>()
                    // change the error too
                    .map_err(|_| ParseFirstItemError::ParseError)
            })
    }

    // A way to write simple code is to `Box<Error>`.
    // The drawback is the error type is only known at runtime.
    fn something() -> Result<i32, Box<dyn error::Error>> {
        "zzz".parse::<i32>()
            .map_err(|e| e.into())  // map error
    }


    // In functions, use `?` operator to return upon error.
    // A `?` is almost exactly the same as an `unwrap()` that `return`ed instead of panicking.
    // In addition, it converts the error to the target type: using the `From` trait.

    impl error::Error for NoFirstElement {} // allows From conversion
    impl error::Error for ParseFirstItemError {} // allows From conversion

    fn parse_second_item(vec: Vec<&str>) -> Result<i32, Box::<dyn error::Error>> {
        let item = vec.first().ok_or(NoFirstElement)?;  // `?` returns error
        let parsed = item.parse::<i32>()?; // `?` returns error
        Ok(2 * parsed)
    }



    // An alternative to Boxing errors (`Box<dyn Error>`) is wrapping them:
    enum DoubleFirstItemError {
        EmptyVec,
        Parse(ParseIntError),
    }

    impl From<ParseIntError> for DoubleFirstItemError {
        fn from(err: ParseIntError) -> Self {
            DoubleFirstItemError::Parse(err)
        }
    }

    fn double_first_item(vec: Vec<&str>) -> Result<i32, DoubleFirstItemError> {
        let first = vec.first().ok_or(DoubleFirstItemError::EmptyVec)?;
        let parsed = first.parse::<i32>()?; // `From` implicitly converts it! Trait `From<ParseIntError>` is implemented for `DoubleError`
        Ok(parsed * 2)
    }
}
