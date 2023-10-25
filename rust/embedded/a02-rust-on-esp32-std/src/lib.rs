// Define the modules' structure
mod lib {
    // Need `pub` -- because otherwise `pub use` won't be able to re-publish them.
    pub mod wifi;
    pub mod httpclient;
}

// Re-publish
pub use crate::lib::{wifi, httpclient};
