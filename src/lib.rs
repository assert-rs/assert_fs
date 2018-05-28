//! Filesystem fixtures and assertions for testing.
//!
//! ```toml
//! [dependencies]
//! assert_fs = "0.1"
//! ```

#![warn(missing_docs)]

extern crate failure;
extern crate globwalk;
extern crate tempfile;

mod fs;
pub use fs::*;

mod errors;
pub use errors::FixtureError;
