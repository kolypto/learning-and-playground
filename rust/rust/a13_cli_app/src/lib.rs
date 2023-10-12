// has: Files & filesystem
use std::fs;

// has: Error
use std::error::Error;

// Logic
// Return value: OK unit, or "trait object" `Box<dyn Error>`: any type that implements the `Error` trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Open the file for reading
    // Don't `.expect()`: rather, return errors with the `?` operator.
    let contents = fs::read_to_string(config.file_path)?;

    // Search
    for line in search(&config.pattern, &contents) {
        println!("{line}");
    }

    Ok(())
}

// Search: pattern in file
// Lifetime `'a`: the result will live as long as the `contents` argument
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Search
    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }

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

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{
            pattern,
            file_path,
        })
    }
}

// App config: command-line input
#[derive(Debug)]
pub struct Config {
    pattern: String,
    file_path: String,
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