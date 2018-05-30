//! Filesystem fixtures and assertions for testing.
//!
//! ```toml
//! [dependencies]
//! assert_fs = "0.1"
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
//! temp.child("foo.txt").touch().unwrap();
//! temp.child("foo.txt").assert(predicate::path::exists());
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
