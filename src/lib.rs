//! Filesystem fixtures and assertions for testing.
//!
//! Fixtures:
//! - A sandbox to work within (see [`TempDir`]).
//! - Setup the sandbox (see [`FileTouch`], [`FileWriteBin`], [`FileWriteStr`], [`PathCopy`]).
//!
//! For assertions, see [`PathAssert`].
//!
//! ```toml
//! [dependencies]
//! assert_fs = "0.9"
//! ```
//!
//! ## Example
//!
//! Here is a trivial example:
//!
//! ```rust
//! extern crate assert_fs;
//! extern crate predicates;
//!
//! use assert_fs::prelude::*;
//! use predicates::prelude::*;
//!
//! let temp = assert_fs::TempDir::new().unwrap();
//! let input_file = temp.child("foo.txt");
//! input_file.touch().unwrap();
//!
//! // ... do something with input_file ...
//!
//! input_file.assert("");
//! temp.child("bar.txt").assert(predicate::path::missing());
//!
//! temp.close().unwrap();
//! ```
//!
//! [`TempDir`]: struct.TempDir.html
//! [`FileTouch`]: trait.FileTouch.html
//! [`FileWriteBin`]: trait.FileWriteBin.html
//! [`FileWriteStr`]: trait.FileWriteStr.html
//! [`PathCopy`]: trait.PathCopy.html
//! [`PathAssert`]: assert/trait.PathAssert.html

#![warn(missing_docs)]

extern crate globwalk;
extern crate predicates;
extern crate predicates_core;
extern crate predicates_tree;
extern crate tempfile;

pub mod assert;
pub mod fixture;
mod errors;

// Pulling this in for convenience-sake
#[doc(inline)]
pub use fixture::TempDir;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use assert::PathAssert;
    pub use fixture::FileTouch;
    pub use fixture::FileWriteBin;
    pub use fixture::FileWriteStr;
    pub use fixture::PathChild;
    pub use fixture::PathCopy;
}
