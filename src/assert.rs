use std::path;

use predicates;

use fs;

/// Assert the state of files within `TempDir`.
///
/// # Examples
///
/// ```rust,ignore
/// use assert_fs::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
/// // ... do something with input_file ...
/// temp.child("bar.txt").assert(predicates::path::missing());
/// temp.close().unwrap();
/// ```
pub trait PathAssert {
    /// Wrap with an interface for that provides assertions on the `TempDir`.
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self;
}

impl PathAssert for fs::TempDir {
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for fs::ChildPath {
    fn assert(&self, pred: &predicates::Predicate<path::Path>) -> &Self {
        assert(self.path(), pred);
        self
    }
}

fn assert(path: &path::Path, pred: &predicates::Predicate<path::Path>) {
    if !pred.eval(path) {
        panic!("Predicate {} failed for {:?}", pred, path);
    }
}
