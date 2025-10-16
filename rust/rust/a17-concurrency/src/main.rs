use std::thread;
use std::time::Duration;


fn main() {
    // === Threads
    // Rust stdlib uses a 1:1 model of thread implementation: one OS thread per one language thread.
    // Why? Because the M:N model requires a runtime, and Rust, being a systems programming language,
    // shouldn't have one.
    // Actually, Rust used to have "green threads" — but they were removed in Rust 1.0.
    // There are crates that implement other models of threading:
    // - Tokio: async tasks multiplexed on thread pool
    // The Go-style M:N threading model never took off in Rust.
    // The decision was intentional—predictable performance, no hidden runtime, easier FFI.

    // `thread::spawn()` starts a closure in a thread.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // `.join()` waits for it to complete.
    // Note that when main() quits, all threads are shut down.
    handle.join().unwrap();

    // The `move` keyword is often used with thread closures: it will take ownership of the values it uses.
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector: {v:?}");
    });
    handle.join().unwrap();


    // Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems
    // were two separate challenges to be solved with different methods.
    // Over time, the team discovered that the ownership and type systems are a powerful set of tools
    // to help manage memory safety *and* concurrency problems!






    // === Messages-Passing
    // Message-sending concurrency: "do not communicate by sharing; share by communicating!"
    // A "channel" is a concept by which data is sent from one thread to another.
    // It has two halves: a transmitter, and a receiver.
    // A channel is said to be "closed" if either the transmitter or receiver half is dropped.

    // MPSC: Multiple-Producers, Single-Consumer channel.
    // It can have multiple senders, but only one receiver.
    use std::sync::mpsc;

    // Get a pair of channels: a tuple (destructure)
    let (tx, rx) = mpsc::channel();

    // Start multiple producers
    for _ in 0..5 {
        // Clone the transmitter
        let tx = tx.clone();

        // Spawn a thread, move the value
        thread::spawn(move || {
            // Send the value.
            // Note: the channel takes ownership of the value. The receiving end will take it afterwards.
            let val = String::from("hey");
            tx.send(val).unwrap();
        });
    }

    // Receive here
    let received = rx.recv().unwrap();
    println!("From channel: {:?}", received);

    // Receive more. Stop when the channel is closed.
    // NOTE: we don't have to wait for the threads to finish: consuming the channel will wait until they're all done!
    drop(tx);  // close the last remaining reference. Otherwise we'll block forever.
    for msg in rx {
        println!("Received: {}", msg);
    }






    // === Shared-State Concurrency
    // "Mutex" only allows one thread to access some data at any given time. The mutex "guards" the data.

    use std::sync::Mutex;
    let m = Mutex::new(5); // the guarded value

    // We start a scope.
    // When the scope ends, the mutable reference is dropped, and the mutex is auto-unlocked!
    {
        // Get a mutable reference.
        // The `.lock()` blocks the current thread until it is able to acquire the mutex.
        // Upon returning, the thread is the only thread with the lock held.
        //
        // Note: `.lock()` will fail if another thread holding the value panicked. No one would able to get the lock.
        // So we use `.unwrap()` to have this thread panic in this situation.
        let mut num = m.lock().unwrap();

        // The type system ensures that you acquire the lock before using the value:
        // you've got to call the `.lock()` method to get the `Mutex<T>` type, which implements `Deref`.
        // It also implements `Drop` that releases the lock automatically when it goes out of scope.
        // This is how borrowing rules + type system remove the risk of forgetting to release the lock :)
        // Now mutate the value:
        *num += 1;
    }

    // Let's share it between threads.
    // We cannot clone the mutex: it won't work.
    // We cannot use `Rc<Mutex<...>>` because `Rc<T>` is not thread-safe. Compiler will give us an error.
    // We need to use `Arc<T>`: atomic reference count.
    use std::sync::Arc;

    // Arc<Mutex> shares the mutex between threads
    let counter = Arc::new(Mutex::new(0)); // shared between threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            // Lock, check, mutate
            *counter.lock().unwrap() += 1
        });
    }

    // Give the threads some time.
    // We could use `.join()` instead.
    thread::sleep(Duration::from_millis(100));

    // See the result
    println!("Result: {}", *counter.lock().unwrap());  //-> 10






    // === Extensible concurrency: the `Sync` and `Send` traits.

    // The `Send` marker trait indicates that ownership of self can be transferred between threads.
    // Almost every Rust type is `Send`.
    // Any type composed entirely of Send types is automatically marked as Send as well.
    // `Rc<T>` is not `Send`: it's not thread-safe.
    use std::marker::Send;

    // The `Sync` marker trait indicates that it is safe for the type to be referenced from multiple threads.
    //
    // Any type `T` implements `Sync` if `&T` (an immutable reference to T) implements `Send`,
    // meaning the reference can be sent safely to another thread. Similar to `Send`,
    //
    // Primitive types are `Sync`, and types composed entirely of `Sync` types are also `Sync`.
    // `Rc<T>` is not `Sync`: it's not thread-safe.
    // `Mutex<T>` is `Sync`.
    use std::marker::Sync;

    // Note: a "marker trait" is a trait with no methods to implement.
}

