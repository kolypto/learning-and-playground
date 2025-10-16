// use: Files & filesystem
use std::fs;

// use: Environment
use std::env;

// use: Error
use std::error::Error;

// Logic
// Return value: OK unit, or "trait object" `Box<dyn Error>`: any type that implements the `Error` trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Open the file for reading
    // Don't `.expect()`: rather, return errors with the `?` operator.
    let contents = fs::read_to_string(config.file_path)?;

    // Search
    for line in search(&config.pattern, &contents, &config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

// Search: pattern in file
// Lifetime `'a`: the result will live as long as the `contents` argument
fn search<'a>(pattern: &str, contents: &'a str, ignore_case: &bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Search
    for line in contents.lines() {
        if *ignore_case && line.to_lowercase().contains(pattern) {
            results.push(line);
        }
        if !*ignore_case && line.to_lowercase().contains(pattern) {
            results.push(line);
        }
    }

    // Alternatively, use functional approach:
    results = contents.lines().filter(|line| line.contains(pattern)).collect();

    // Done
    results
}


// Config methods
impl Config {
    // Config builder
    // The returned error string is `'static` because we actually return a static (i.e. literal) string.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < (2+1) {
            return Err("not enough arguments");
        }

        // clone() fixes ownership problems — but has runtime cost.
        // Developers avoid using clone() for more efficient methods.
        let pattern = args[1].clone();
        let file_path = args[2].clone();

        // Read env
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Done
        Ok(Config{
            pattern,
            file_path,
            ignore_case,
        })
    }

    // An improved version of `new()` using iterators that don't need `clone()`:
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // Skip arg[0]
        args.next();

        // arg[1]: pattern
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // arg[2]: file path
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pattern,
            file_path,
            ignore_case,
        })
    }
}

// App config: command-line input
#[derive(Debug)]
pub struct Config {
    pattern: String,
    file_path: String,
    ignore_case: bool,
}



// Unit-test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
";
        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }
}