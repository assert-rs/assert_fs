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
//! assert_fs = "0.10"
//! ```
//!
//! ## Overview
//!
//! Setting up a fixture
//! - [`TempDir`]
//! - [`touch`][`FileTouch`] a [`ChildPath`]
//! - [`write_binary`][`FileWriteBin`] a [`ChildPath`]
//! - [`write_str`][`FileWriteStr`] a [`ChildPath`]
//! - [`copy_from`][`PathCopy`] a pristine folder to a [`ChildPath`] or [`TempDir`]
//!
//! Validating
//! - [`assert`][`PathAssert`] a [`ChildPath`] or [`TempDir`]
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
//! ## Relevant crates
//!
//! Other crates that might be useful in testing command line programs.
//!
//! [`TempDir`]: struct.TempDir.html
//! [`ChildPath`]: fixture/struct.ChildPath.html
//! [`FileTouch`]: fixture/trait.FileTouch.html
//! [`FileWriteBin`]: fixture/trait.FileWriteBin.html
//! [`FileWriteStr`]: fixture/trait.FileWriteStr.html
//! [`PathCopy`]: fixture/trait.PathCopy.html
//! [`PathAssert`]: assert/trait.PathAssert.html
//! [dir-diff]: https://crates.io/crates/dir-diff

#![warn(missing_docs)]

extern crate globwalk;
extern crate predicates;
extern crate predicates_core;
extern crate predicates_tree;
extern crate tempfile;

pub mod assert;
mod errors;
pub mod fixture;

// Pulling this in for convenience-sake
#[doc(inline)]
pub use fixture::TempDir;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use assert::PathAssert;
    pub use fixture::PathCreateDir;
    pub use fixture::FileTouch;
    pub use fixture::FileWriteBin;
    pub use fixture::FileWriteStr;
    pub use fixture::PathChild;
    pub use fixture::PathCopy;
}
