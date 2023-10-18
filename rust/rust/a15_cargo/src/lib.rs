//! # My crate
//!
//! This is a doccomment for the file
//! It adds documentation to the item that contains it: file (the crate) or the module.

/*! The so-called "inner doc comment" (for the crate)
 * can also look like this
 */

/// Doccomments. Start with triple /
///
/// # Examples
/// Here we go with Markdown.
/// This Markdown will be used on crates.io to document the usage of your library.
///
/// This code will actually be run as a unit-test:
///
/// ```rust
/// # // line prefixed with `#` is ignored in the documetation, but is still tested!
/// let arg = 5;
/// let answer = a15_cargo::add_one(arg);
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
/// never panics
///
/// # Errors
/// Describe the kinds of errors that might occur
///
/// # Safety
/// If the function is `unsafe` to call, explain it
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** This is another type of "outer doc comment":
 * It documents a function while being outside of it
*/
pub fn something(){}




// So:
//  //!      Inner doc comment
//  /*! */   Inner doc comment
//  ///      Outer doc comment
//  /**  */  Outer doc comment




// We can generate HTML from this documentation into `target/doc` using
// $ cargo doc
// To open it in the browser:
// $ cargo doc --open

// Note: `cargo test` will run the code examples in the documentation as tests!
// $ cargo test



// A published module
pub mod artistic {
    //! Supplementary features

    /// Primary colors in the RYB model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}

// Re-export a type/function for convenience
pub use self::artistic::PrimaryColor;