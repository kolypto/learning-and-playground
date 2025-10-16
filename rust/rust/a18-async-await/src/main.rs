fn main() -> Result<(), Box<dyn Error>> {
    // Get args
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..]; // skip the 1st one

    // Async code needs a *runtime*: a Rust crate that manages the details of executing async code.
    // Rust does not bundle a runtime: instead, there are many different runtimes available, each
    // with different tradeoffs suitable to the use case it targets: huge server or tiny microcontroller.
    // A runtime executes async functions -- which are hidden state machines.
    //
    // Under the hood, every `Future` has a `poll()` method that returns:
    // are you ready? or still pending? But polling in a loop would be a waste!
    // Runtimes (like tokio) utilize OS-level I/O readiness mechanisms like epoll/kqueue/select.
    // The `poll()` method is only called when the underlying resources is likely ready:
    // the runtime registers a Waker with the driver that will wake up the future.
    use tokio::runtime::Runtime;
    let r = Runtime::new()?;


    // Fetch one URL to test
    let title = r.block_on(
        a18_async_await::fetch_page_title(&args[0])
    )?.expect("page has no title");
    println!("Test fetch: {title}");

    // Race our two URLs
    let title = r.block_on(async {
        // Futures are only defined and not executed until they're awaited on.
        let title1 = fetch_page_title(&args[0]);
        let title2= fetch_page_title(&args[1]);

        // Pin.
        //
        // Pin puts a value to the stack so it can't be moved in memory.
        // Pin is a wrapper for pointer-like types such as &, &mut, Box, and Rc.
        // (Technically, Pin works with types that implement the Deref or DerefMut traits, but this is effectively the same thing)
        // It makes sure that the value (the Future's state machine with await points and state)
        // remains at the same place in memory and is not moved: e.g.
        // when you move it into a `Vec` or pass it to `join_all` or even return from a function.
        //
        // Normally, Rust handles moves itself — but async state machines are a hack.
        // And Rust won't introduce hidden costs: that's why you have to `pin!()` manually.
        // So: a Pin is when you move the pointer around, but the data it points to is in the same place.
        //
        // Most types are perfectly safe to move around. They have the `Unpin` marker trait.
        // We only have to pin where items have internal references.
        // Pinning is required for some futures that are self-referential: with pointers
        // to their own fields. Without pin!(), you'd need `Box::pin(fut)` to heap-allocate it.
        let title1 = pin!(title1);
        let title2 = pin!(title2);

        // select() waits for either one of two Futures to complete
        match future::select(title1, title2).await {
            Either::Left((a, _f2)) => a,
            Either::Right((b, _f1)) => b,
        }
    })?;

    // Print it
    match title {
        Some(title) => println!("Page title: {title}"),
        None => println!("Page had no title"),
    };

    // Use channels to communicate
    r.block_on(async{
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Work in the background.
        // We ignore the return value: join handle.
        let tx_fut = async move {
            for i in 0..3 {
                tx.send(i.to_string()).unwrap();  // doesn't block.
                tokio::time::sleep(Duration::from_millis(250)).await;
            }
            // We don't have to drop it because it goes out of scope automatically:
            // thanks to "move" block!
            // drop(tx);
        };

        // Receive until exhausted
        let rx_fut = async {
            // recv() is async
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // Join
        futures::join!(tx_fut, rx_fut);
        println!("Done");
    });

    // Spawn N worker threads
    r.block_on(async{
        // Spawn tasks: a background worker starts immediately on the executor's thread pool.
        // It ay execute on the current thread, or it may be sent to a different thread to be executed.
        // If you `await`, that's like `join()`ing a thread.
        // NOTE: no need to `pin!()`: `spawn()` pins internally.
        // Key difference: spawn requires 'static lifetime (no borrowed data), because
        // the task might outlive the current scope.
        // Use spawn for parallelism, async blocks for composition.
        let mut futures = Vec::new();
        for i in 0..10 {
            let fut = tokio::task::spawn(async move {
                println!("task {i}");
            });
            futures.push(fut);
        }

        // Wait on them
        futures::future::join_all(futures).await;
    });

    // Collect results from futures:
    r.block_on(async{
        let a = async { 1u32 };
        let b = async { "Hello!" };

        // Collect results
        let (result_a, result_b) = futures::join!(a, b);
        println!("results: {result_a}, {result_b}");
    });

    // More info
    r.block_on(streams())?;

    // Done
    Ok(())
}


use std::error::Error;
use std::time::Duration;
use std::vec;
use std::{pin::pin};
use std::future::Future;

use a18_async_await::fetch_page_title;

use futures::channel::mpsc::UnboundedReceiver;
use futures::{
    future::{self, Either},
};
use tokio;
use tokio_stream::{self, StreamExt};

async fn streams() -> Result<(), Box<dyn Error>> {
    // === Streams === //
    // The async `recv()` method that we used is known as a *Stream pattern*.
    // It's like iterator's `.next()` method: `async recv()`.
    //
    // Under the hool, Streams also use polling: `poll_next()` (different method)

    // This means that you can create a stream from an iterator:
    let list = vec![1,2,3];
    let iter = list.iter().map(|x| x*2);
    let stream = tokio_stream::iter(iter); // convert to stream

    // Filter stream. Yes, async.
    let mut stream = stream.filter(|x| x % 3 == 0);

    // Note: methods from traits can only be used if they're in scope.
    // We use StreamExt to get additional methods on the stream.
    use tokio_stream::StreamExt;
    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }

    // Unique stream features: composing streams.
    let letters = vec2stream(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let numbers = vec2stream(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    // Merge streams
    // It actually has many Rx features: like `map()`, `throttle()`, ...
    let stream = letters.merge(numbers);

    // Print them
    // Stream with a timeout. It comes from `StreamExt` trait.
    let mut stream = pin!(stream.timeout(Duration::from_millis(300)));
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("message={message}"),
            Err(reason) => eprintln!("Problem: {reason:?}"),
        }
    }

    // Done
    Ok(())
}


// Convert: vector to stream
// It creates a channel and returns the `rx` end: the stream.
fn vec2stream(messages: Vec<String>) -> impl futures::Stream<Item = String> {
    // tx, rx
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    // Send messages
    tokio::task::spawn(async move {
        for message in messages {
            tx.send(message);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // Return
    tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
}
