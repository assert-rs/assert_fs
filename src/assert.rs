//! Filesystem assertions.

use std::fmt;
use std::path;

use predicates;
use predicates::path::PredicateFileContentExt;
use predicates::str::PredicateStrExt;

use fs;

/// Assert the state of files within `TempDir`.
///
/// # Examples
///
/// ```rust,ignore
/// use assert_fs::*;
/// use predicates::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
/// // ... do something with input_file ...
/// input_file.assert("");
/// temp.child("bar.txt").assert(predicate::path::missing());
/// temp.close().unwrap();
/// ```
pub trait PathAssert {
    /// Wrap with an interface for that provides assertions on the `TempDir`.
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>;
}

impl PathAssert for fs::TempDir {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for fs::ChildPath {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

fn assert<I, P>(path: &path::Path, pred: I)
where
    I: IntoPathPredicate<P>,
    P: predicates::Predicate<path::Path>,
{
    let pred = pred.into_path();
    if !pred.eval(path) {
        panic!("Predicate {} failed for {:?}", pred, path);
    }
}

/// Used by `PathAssert` to convert Self into the needed `Predicate<Path>`.
pub trait IntoPathPredicate<P>
where
    P: predicates::Predicate<path::Path>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a path.
    fn into_path(self) -> P;
}

impl<P> IntoPathPredicate<P> for P
where
    P: predicates::Predicate<path::Path>,
{
    type Predicate = P;

    fn into_path(self) -> Self::Predicate {
        self
    }
}

// Keep `predicates` concrete Predicates out of our public API.
/// Predicate used by `IntoPathPredicate` for `str`
#[derive(Debug, Clone)]
pub struct StrContentPathPredicate(
    predicates::path::FileContentPredicate<
        predicates::str::Utf8Predicate<predicates::str::DifferencePredicate>,
    >,
);

impl StrContentPathPredicate {
    pub(crate) fn new(value: &'static str) -> Self {
        let pred = predicates::str::similar(value).from_utf8().from_file_path();
        StrContentPathPredicate(pred)
    }
}

impl predicates::Predicate<path::Path> for StrContentPathPredicate {
    fn eval(&self, item: &path::Path) -> bool {
        self.0.eval(item)
    }
}

impl fmt::Display for StrContentPathPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoPathPredicate<StrContentPathPredicate> for &'static str {
    type Predicate = StrContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use predicates::prelude::*;

    // Since IntoPathPredicate exists solely for conversion, test it under that scenario to ensure
    // it works as expected.
    fn convert_path<I, P>(pred: I) -> P
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>,
    {
        pred.into_path()
    }

    #[test]
    fn into_path_from_pred() {
        let pred = convert_path(predicate::eq(path::Path::new("hello.md")));
        assert!(pred.eval(path::Path::new("hello.md")));
    }

    #[test]
    fn into_path_from_str() {
        let pred = convert_path("hello\n");
        assert!(pred.eval(path::Path::new("tests/fixture/hello.txt")));
    }
}
