//! Filesystem fixtures and assertions for testing.
//!
//! ```toml
//! [dependencies]
//! assert_fs = "0.1"
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
