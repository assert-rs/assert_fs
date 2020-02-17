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
//! assert_fs = "0.13"
//! ```
//!
//! ## Overview
//!
//! Setting up a fixture
//! - [`TempDir`] or [`NamedTempFile`]
//! - [`touch`][`FileTouch`] a [`ChildPath`] or [`NamedTempFile`]
//! - [`write_binary`][`FileWriteBin`] a [`ChildPath`] or [`NamedTempFile`]
//! - [`write_str`][`FileWriteStr`] a [`ChildPath`] or [`NamedTempFile`]
//! - [`write_file`][`FileWriteFile`] a [`ChildPath`] or [`NamedTempFile`]
//! - [`copy_from`][`PathCopy`] a pristine folder to a [`ChildPath`] or [`TempDir`]
//!
//! Validating
//! - [`assert`][`PathAssert`] a [`ChildPath`], [`TempDir`], or [`NamedTempFile`]
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
//! [`NamedTempFile`]: struct.NamedTempFile.html
//! [`ChildPath`]: fixture/struct.ChildPath.html
//! [`FileTouch`]: fixture/trait.FileTouch.html
//! [`FileWriteBin`]: fixture/trait.FileWriteBin.html
//! [`FileWriteStr`]: fixture/trait.FileWriteStr.html
//! [`FileWriteFile`]: fixture/trait.FileWriteFile.html
//! [`PathCopy`]: fixture/trait.PathCopy.html
//! [`PathAssert`]: assert/trait.PathAssert.html
//! [dir-diff]: https://crates.io/crates/dir-diff

#![warn(missing_docs)]

pub mod assert;
pub mod fixture;

// Pulling this in for convenience-sake
#[doc(inline)]
pub use crate::fixture::TempDir;

// Pulling this in for convenience-sake
#[doc(inline)]
pub use crate::fixture::NamedTempFile;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use crate::assert::PathAssert;
    pub use crate::fixture::FileTouch;
    pub use crate::fixture::FileWriteBin;
    pub use crate::fixture::FileWriteFile;
    pub use crate::fixture::FileWriteStr;
    pub use crate::fixture::PathChild;
    pub use crate::fixture::PathCopy;
    pub use crate::fixture::PathCreateDir;
}

#[macro_use]
extern crate doc_comment;
doctest!("../README.md");
