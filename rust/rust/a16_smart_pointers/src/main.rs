fn main() {
    // Smart pointers: data structures that act like a pointer, but also have additional capabilities.
    // E.g. `Rc`: reference counting smart pointer that allows data to have multiple owners.

    // In many cases, smart pointers *own* the data they point to.
    // `String` and `Vec<T>` count as smart pointers because they own some memory
    // and allow you to manipulate it.
    // As for extra capacities: `String` stores its capacity, and has the ability to ensure valid UTF-8;

    // Smart pointers implement `Deref` and `Drop` traits:
    // * `Deref` allows an instance of the smart pointer (usually a struct) to behave like a reference
    // * `Drop` allows you to customize the code that's run when an instance goes out of scope (destructor?)

    // Many libraries have their own smart pointers, but here are the most common ones:
    // * `Box<T>` for allocating values on the heap
    // * `Rc<T>` a reference counting type that enables multiple ownership
    // * `Ref<T>` and `RefMut<T>`: enforces the borrowing rules at runtime instead of compile time.
    //   They are accessed through `RefCell<T>`
    // * "Interior mutability": when an immutable type exposes and API for mutating an interior value


    // An overview:
    // * `Rc<T>` enables multiple owners of the same data;
    //   `Box<T>` and `RefCell<T>` have single owners.
    // * `Box<T>` allows immutable or mutable borrows checked at compile time;
    //   `Rc<T>` only allows immutable borrows checked at compile time;
    //   `RefCell<T>` allows immutable or mutable borrows checked *at run time*.
    // * With `RefCell<T>`, you can mutate the value inside it even when the `RefCell<T>` is immutable.




    // === Box<T>
    // Boxes allow you to store data on the heap rather than the stack. The stack will only have a pointer to the heap.
    // Boxes don't have performance overhead (other than storing their data on the heap).
    // Use boxes when:
    // * When you have a type whose size can't be known at compile time
    // * When you have a large amount of data and want to transfer ownership without copying data
    // * When you want a value of any type, but with a specific trait

    // Store this `i32` on the heap.
    // When the owned Box goes out of scope, it will be deallocated.
    let _ = Box::new(5 as i32);  // example: type casting

    // Boxes enable "recursive types": a type that can have another value of the same type as part of itself.
    // Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.
    // Example: cons list. It comes from Lisp, and is a version of a linked list:
    // each item contains 2 elements: [the value of the current item, next item]. Recursion ends with `Nil`.
    //  (1, 2, (3, Nil))

    enum List {
        // This naive example will fail: "recursive type `List` has infinite size": "recursive without indirection"
        // This is because Rust can't figure out how much space it needs to store a `List` value.
        //Cons(i32, List),  //UNC

        // The corrected version.
        // It works because the `Box<T>` pointer takes const space no matter what data is in there.
        Cons(i32, Box<List>),

        Nil,
    }
    use List::{Cons, Nil}; // import nested `Cons` and `Nil` into this scope

    let _ = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );



    // The "dereference operator" `*`:
    let x = 5;
    let y = &x; // a reference: a pointer to a value stored elsewhere
    assert_eq!(5, *y); // get the value: "dereference"
    //assert_eq!(5, y); // Error: help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`  //UNC


    // Boxes implement the `Deref` trait, which allows `Box<T>` values to be treated like references.
    // The `Deref` trait allows you to customize the behavior of the "dereference operator" `*`.
    // Using `Box<T>` like a reference:
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);  // dereference



    // Let's build our own smart pointer: to see how it works.
    struct MyBox<T>(T); // a tuple

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {  // implement the `Deref` trait
        // Defines an associated type for the `Deref` trait to use: it's like a generic parameter
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // And use it
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);  // dereference. Behind the scenes: `*(y.deref())`



    // The `Drop` trait: run code when a value is about to go out of scope.
    // The compiler will insert this code automatically.
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // Rust won't let you call `.drop()` manually because it will run this function itself. It'd be a double-free.
    // Use `std::mem::drop()` function if you must:
    let c = CustomSmartPointer{
        data: String::from("hey"),
    };
    drop(c);






    // === Rc<T>
    // Reference-counting smart pointer.
    // Used in cases where a single value might have multiple owners:
    // E.g. in graph data structures, where multiple edges might point to the same node:
    // a node shouldn't be cleaned up unless it doesn't have any edges pointing to it and so has no owners.

    // The `Rc<T>` type keeps track of the number of references to a value to determine
    // whether or not the value is still in use. The value is cleaned up when there are 0 references.

    // Example: a person enters the TV room and turn it on. Others come in and join: the TV is still on.
    // When the last person leaves the room, they turn the TV off: it's no longer being used.
    // But if someone turns the TV off while others are still watching, there would be an uproar!! :)

    // ❗ NOTE: `Rc<T>` is only for use in single-threaded scenarios!!
    // See Chapter 16: using `Rc<T>` in multithreaded systems.

    // Let's implement a value that can be added to two lists:
    let two = Box::new(2); // a value
    let _list_a = vec![Box::new(1), two, Box::new(3)];
    // let _list_b = vec![Box::new(1), two, Box::new(3)];  // ERROR: "use of moved value: `two`"  //UNC

    // it fails because `list_a` owns the value of `two`: no one else can own it
    use std::rc::Rc;
    let two = Rc::new(2); // a value
    let _list_a = vec![Rc::new(1), Rc::clone(&two), Rc::new(3)];
    let _list_b = vec![Rc::new(1), Rc::clone(&two), Rc::new(3)];  // Rc::clone() increments the reference count

    // NOTE: two.clone() is also ok, but for most types, it creates a deep copy.
    // The convention is to use Rc::clone() that always makes a shallow copy. It's cheaper.

    // See the reference count
    println!("RC count for `two`: {}", Rc::strong_count(&two));  //-> 3

    // NOTE: `Rc<T>` returns immutable references. You cannot modify the value. It's read-only.
    // For mutable refrences, see `RefCell<T>`






    // === RefCell<T>
    // "Interior mutability" allows you to mutate data even when there are immutable references to that data.
    // This is normally disallowed, but this pattern uses `unsafe` code to bend the usual rules:
    // code marked as `unsafe` promises to check the rules manually.
    // So: with `RefCell<T>`, you can mutate the value inside it even when the `RefCell<T>` is immutable.

    // This pattern is useful when the conservative compiler doesn't let you do something that you're sure
    // you can safely handle yourself. The compiler is "conservative" in that it would rather reject
    // a correct program than allow an incorrect one. The `RefCell<T>` lets you bend the rules.

    // Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds.
    // This means that:
    // * At any given time, you can *either* have one mutable reference, or many immutable references
    // * References must always be valid

    // With `Box<T>`, the borrowing rules are enforced at compile time;
    // whereas with `RefCell<T>` the borrowing rules are enforced *at runtime*.
    // If you break the `RefCell<T>` rules, your program will panic!

    // The internal code of `RefCell<T>` is `unsafe`, but the API is safe.
    // ❗ NOTE: `RefCell<T>` is only for use in single-threaded scenarios!!
    // ❗ See `Mutex<T>`: the thread-safe version of RefCell<T>


    // A use case for interior mutability: Mock Objects.
    // Let's first implement an object, `LimitTracker`, that checks how close a user is to a quota:

    /// Notifies the user: SMS or something
    pub trait Messenger {
        /// Send a notification, e.g. SMS
        fn send(&self, msg: &str);  // takes an immutable reference to `&self`, and to the text
    }

    /// Limit checker: if a user is close to a limit, send a notification.
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: u64,
        max: u64,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        /// Constructor
        pub fn new(messenger: &'a T, max: u64) -> LimitTracker<'a, T> {
            LimitTracker{messenger, value: 0, max}
        }

        /// Set the new value.
        /// Notify the user if they're close to the quota.
        pub fn set_value(&mut self, value: u64){
            // Set the value
            self.value = value;

            // Usage
            let perc = self.value as f64 / self.max as f64;

            // Check quota
            if perc >= 1.0 {
                self.messenger.send("Error: over quota!");
            } else if perc >= 0.9 {
                self.messenger.send("Warning: quota almost used up!");
            }
        }
    }

    // Now, the mock object.
    // It needs to implement the trait.
    use std::cell::RefCell;

    struct MockMessenger {
        // The interface defines `send(&self)` as an immutable reference.
        // But we need a mutable reference that will let us modify the value.
        // This is where interior mutability can help!
        // We'll store `sent_messages` within a RefCell<T>, which is itself immutable,
        // but the value that it points to is mutable.
        // sent_messages: Vec<String>,  // error: "`self` is a `&` reference, so the data it refers to cannot be borrowed as mutable"  //UNC
        sent_messages: RefCell<Vec<String>>, // OK
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                // Fails
                // sent_messages: vec![],  //UNC
                // Works
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Fails
            // self.sent_messages.push(String::from(message));  //UNC
            // Works.
            // `.borrow_mut()` gets a mutable reference, which is returned when dropped
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }


    // Use it
    let messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&messenger, 100);

    limit_tracker.set_value(95);
    // `.borrow()` gets an immutable reference
    assert_eq!(messenger.sent_messages.borrow().len(), 1);


    // Here is how it works:
    // the `RefCell<T>` keeps track of how many `.borrow()` (`Ref<T>`) and `.borrow_mut()` (`RefMut<T>`)
    // smart pointers are currently active. When a value gets out of scope (dropped), the count decrements.
    // It lets you have many immutable borrows, or one mutable borrow.
    // If you try to violate the rules, it panics.






    // === Having multiple owners of mutable data
    // Recall: `Rc<T>` lets you have multiple owners of immutable data.
    // Recall: `RefCell<T>` is immutable. So you can share multiple references.
    // By combining `Rc<T>` and `RefCell<T>`, you can share mutable data by allowing multiple references!

    // Two lists will point to a shared mutable value
    let two = Rc::new(RefCell::new(2));
    let list_a = vec![
        Rc::new(RefCell::new(1)),
        Rc::clone(&two),
        Rc::new(RefCell::new(3)),
    ];
    let list_b = vec![
        Rc::new(RefCell::new(1)),
        Rc::clone(&two),
        Rc::new(RefCell::new(3)),
    ];

    // Mutate it
    *two.borrow_mut() += 10;

    // Check
    println!("list_a={:?}", list_a);  //-> list_a=[RefCell { value: 1 }, RefCell { value: 12 }, RefCell { value: 3 }]
    println!("list_b={:?}", list_b);  //-> list_b=[RefCell { value: 1 }, RefCell { value: 12 }, RefCell { value: 3 }]






    // === Weak references
    use std::rc::Weak;
    // `Rc<T>` may form reference cycles that are never cleaned up: rc count never drops down to zero.
    // To prevent reference cycles, turn `Rc<T>` into a `Weak<T>`:
    // use `Rc::downgrade()` to get a weak reference. It does not express an ownership relationship:
    // their count does not affect when an `Rc<T> instance is cleaned up.
    // A weak reference increases the `weak_count` of an `Rc<T>`. They do not prevent cleaning up.

    // To get a value from a weak reference, call its `.upgrade()` method to get an `Option<Rc<T>>`:
    // it returns a reference, or `None` if the value has been dropped already.

    // Let's create a tree whose items know about their children *and* their parents.
    #[derive(Debug)]
    struct Node {
        // Node value
        value: i64,

        // Mutable reference to a list of children, where every child is represented as an `Rc<Node>`.
        // When creating this object, we will need to set the children, and then put the same values into parents.
        // To allow that, we need:
        // * `Rc<Node>` to share ownership (with the calling code), and to allow weak references
        // * `RefCell<Node>` to make mutable
        //
        // To allow that, we need
        children: RefCell<Vec<Rc<Node>>>,

        // Parents: weak references to Nodes
        // ❗ We cannot use strong references because that would create a reference cycle
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    let branch = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),  // leaf Rc
        parent: RefCell::new(Weak::new()),
    });

    // update the parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

}
