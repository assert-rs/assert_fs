//! Filesystem fixtures and assertions for testing.
//!
//! ```toml
//! [dependencies]
//! assert_fs = "0.2"
//! ```
//!
//! ## Example
//!
//! Here is a trivial example:
//!
//! ```rust,ignore
//! use assert_fs::prelude::*;
//! use predicates::prelude::*;
//!
//! let temp = assert_fs::TempDir::new().unwrap();
//! let input_file = temp.child("foo.txt");
//! input_file.touch().unwrap();
//! // ... do something with input_file ...
//! input_file.assert(predicate::path::exists());
//! temp.child("bar.txt").assert(predicate::path::missing());
//! temp.close().unwrap();
//! ```

#![warn(missing_docs)]

extern crate failure;
extern crate globwalk;
extern crate predicates;
extern crate tempfile;

mod assert;
pub use assert::*;

mod fs;
pub use fs::*;

mod errors;
pub use errors::FixtureError;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use assert::TempDirAssertExt;
    pub use fs::ChildPathTouchExt;
    pub use fs::ChildPathWriteBinExt;
    pub use fs::ChildPathWriteStrExt;
    pub use fs::TempDirChildExt;
    pub use fs::TempDirCopyExt;
}
