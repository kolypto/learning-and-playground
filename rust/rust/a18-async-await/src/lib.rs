// Alternative approach to asynchronous programming:
// Futures, Streams, async/await.

// Differences between parallelism and concurrency:
// - Concurrency: switching between tasks.                  Example: single-core CPU.
// - Parallelism: executing tasks at exactly the same time. Example:  multi-core CPU
// Async in Rust is concurrency.
// It can, however, use parallelism under the hood (e.g. threadpool)


//=== Future ===//
// A *future* is a value that may not be ready now — but will become ready
// at some point in the future. Each future holds its own information about the progress
// that has been made ans what "ready" means.
//
// Rust provides a `Future` trait to be used as an interface.
// You can implement your own data types.
//
// You can apply the `async` keyword to blocks and functions to specify that they can be
// interrupted and resumed.
// Within an `async` block or function you can use the `await` keyword to *await a future*:
// that is, wait for it to become ready. This is where functions can be paused & resumed.


// Our First Async Program: a little web scraper.
// We'll pass in two URLs from the command line and fetch both concurrently,
// then pull their <title> and print out whichever finishes first.

// Use: HTTP requests
// $ cargo get reqwest
use reqwest;


/// Takes an URL, fetches it, and returns the text in the <title> element
pub async fn fetch_page_title(url: &str) -> Result<Option<String>, reqwest::Error> {
    // Load page
    // ❗ NOTE: Futures in Rust are *lazy*: they won't do anything unless you `await` on them.
    // This is different from how many other languages approach async!
    //
    // NOTE: In Rust, `await` is a postfix keyword! Uncommon, but allows nicer chaining.
    // Every `await point` is a place where control is handed back to the runtime.
    // It's actually a state machine that can suspend and result:
    // the Rust compiler transforms async functions into state machines:
    // (init, await point 1, await point 2, ..., done)
    let response = reqwest::get(url).await?;

    // Get text
    // The method is also async because we have to wait for the entire response to arrive.
    let response_text = response.text().await?;

    // Parse HTML
    Ok(extract_title(&response_text))
}

// This is what we have done.
//
// When Rust sees a block marked with the `async` keyword, it compiles it
// into a unique, anonymous data type that implements the Future trait.
//
// When Rust sees a function marked with `async`, it compiles it into a non-async function
// whose body is an async block: i.e. it returns that anonymous data type with Future trait.
//
// Thus, writing an async fn is equivalent to writing a function that returns
// a *future* of the return type:
pub fn fetch_page_title_async(url: &str) -> impl Future<Output = Result<Option<String>, reqwest::Error>> {
    async move {
        fetch_page_title(url).await
    }
}


/// Extract <title> from HTML string
fn extract_title(html: &str) -> Option<String> {
    // TODO: use "scrape" create with xpath query
    use regex::Regex;
    let re = Regex::new(r"(?i)<title>(.*?)</title>").unwrap();
    re.captures(html)?.get(1).map(|m| m.as_str().to_string())
}
