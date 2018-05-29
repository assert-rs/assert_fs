use std::path;

use predicates;

use fs;

/// Extend `TempDir` with assertions.
///
/// # Examples
///
/// ```rust,ignore
/// use assert_fs::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// temp.child("foo.txt").touch().unwrap();
/// temp.child("foo.txt").assert(predicates::path::exists());
/// temp.child("bar.txt").assert(predicates::path::missing());
/// temp.close().unwrap();
/// ```
pub trait TempDirAssertExt {
    /// Wrap with an interface for that provides assertions on the `TempDir`.
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self;
}

impl TempDirAssertExt for fs::TempDir {
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self {
        if !pred.eval(self.path()) {
            panic!("Predicate failed for {:?}", self.path());
        }
        self
    }
}

impl TempDirAssertExt for fs::ChildPath {
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self {
        if !pred.eval(self.path()) {
            panic!("Predicate failed for {:?}", self.path());
        }
        self
    }
}
