// use: Environment and arguments
use std::env;

// use: Exit()
use std::process;

// Our lib: we use `Config`, `run()`
use a13_command_line_app as minigrep;

fn main() {
    // Get cmdline arguments.
    // NOTE: it will panic if argument contains invalid Unicode.
    // To accept invalid unicode, use `args_os()` instead.
    let args: Vec<String> = dbg!(env::args().collect());

    // Config(): our library func that returns a Config
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        // `eprintln!()` prints to stderr
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        process::exit(255);
    });
    println!("config={config:?}");

    // Logic moved into lib.rs
    // run() the logic
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}