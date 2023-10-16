fn main() {
    // Rust is object oriented, in a way:
    // structs and enums have data, and `impl` blocks provide methods on them.
    // Methods in Rust can be `pub`: this is the only public API that users can see.

    // Rust has no inheritance.
    // For code reuse, some limited inheritance is available with default trait methods.

    // Inheritance has recently fallen out of favor as a programming design solution:
    // it's all too easy to share more code than necessary. In addition, some languages
    // only allow single inheritance, which is also quite limited.
    // For these reasons, Rust uses trait objects instead of inheritance.

    // "Trait objects" allow for values of different types: it's an interface.
    // The concept of Rust is: "duck typing". The interface matters, the type -- not as much.
    // Define a trait:
    pub trait Draw {
        fn draw(&self);
    }

    // Define a struct
    pub struct Screen {
        // A list of references to "trait objects": any object that implements the trait.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self){
            for component in self.components.iter() {
                // Trait objects peform "dynamic dispatch": the compiler can't tell which method that is,
                // so it emits code that figures out the method to call at runtime.
                // This lookup incurs a runtime cost. It also prevents the compiler from inlining the method.
                component.draw();
            }
        }
    }

    // Implement a `Draw` type
    pub struct Button {
        pub w: u32,
        pub h: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self){
            // ... draw code
        }
    }

    // Now we can build the GUI:
    let screen = Screen {
        components: vec![
            Box::new(Button{w: 75, h: 10, label: String::from("OK")}),
        ],
    };
    screen.run();

}
