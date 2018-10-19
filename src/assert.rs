//! Filesystem assertions.
//!
//! See [`PathAssert`].
//!
//! # Examples
//!
//! ```rust,ignore
//! use assert_fs::*;
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
//! [`PathAssert`]: trait.PathAssert.html

use std::fmt;
use std::path;

use predicates;
use predicates::path::PredicateFileContentExt;
use predicates::str::PredicateStrExt;
use predicates_core;
use predicates_tree::CaseTreeExt;

use fixture;

/// Assert the state of files within [`TempDir`].
///
/// # Examples
///
/// ```rust,ignore
/// use assert_fs::*;
/// use predicates::prelude::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert("");
/// temp.child("bar.txt").assert(predicate::path::missing());
///
/// temp.close().unwrap();
/// ```
///
/// - See [`predicates::prelude`] for more predicates.
/// - See [`IntoPathPredicate`] for other built-in conversions.
///
/// [`TempDir`]: ../struct.TempDir.html
/// [`predicates::prelude`]: https://docs.rs/predicates/0.9.0/predicates/prelude/
/// [`IntoPathPredicate`]: trait.IntoPathPredicate.html
pub trait PathAssert {
    /// Wrap with an interface for that provides assertions on the `TempDir`.
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates_core::Predicate<path::Path>;
}

impl PathAssert for fixture::TempDir {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates_core::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for fixture::ChildPath {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates_core::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

fn assert<I, P>(path: &path::Path, pred: I)
where
    I: IntoPathPredicate<P>,
    P: predicates_core::Predicate<path::Path>,
{
    let pred = pred.into_path();
    if let Some(case) = pred.find_case(false, &path) {
        panic!("Unexpected file, failed {}\npath={:?}", case.tree(), path);
    }
}

/// Used by [`PathAssert`] to convert Self into the needed [`Predicate<Path>`].
///
/// [`PathAssert`]: trait.PathAssert.html
/// [`Predicate<Path>`]: https://docs.rs/predicates-core/0.9.0/predicates_core/trait.Predicate.html
pub trait IntoPathPredicate<P>
where
    P: predicates_core::Predicate<path::Path>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a path.
    fn into_path(self) -> P;
}

impl<P> IntoPathPredicate<P> for P
where
    P: predicates_core::Predicate<path::Path>,
{
    type Predicate = P;

    fn into_path(self) -> Self::Predicate {
        self
    }
}

// Keep `predicates` concrete Predicates out of our public API.
/// Predicate used by `IntoPathPredicate` for bytes
#[derive(Debug)]
pub struct BytesContentPathPredicate(
    predicates::path::FileContentPredicate<predicates::ord::EqPredicate<&'static [u8]>>,
);

impl BytesContentPathPredicate {
    pub(crate) fn new(value: &'static [u8]) -> Self {
        let pred = predicates::ord::eq(value).from_file_path();
        BytesContentPathPredicate(pred)
    }
}

impl predicates_core::reflection::PredicateReflection for BytesContentPathPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl predicates_core::Predicate<path::Path> for BytesContentPathPredicate {
    fn eval(&self, item: &path::Path) -> bool {
        self.0.eval(item)
    }
}

impl fmt::Display for BytesContentPathPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoPathPredicate<BytesContentPathPredicate> for &'static [u8] {
    type Predicate = BytesContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
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

impl predicates_core::reflection::PredicateReflection for StrContentPathPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl predicates_core::Predicate<path::Path> for StrContentPathPredicate {
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

// Keep `predicates` concrete Predicates out of our public API.
/// Predicate used by `IntoPathPredicate` for `str`
#[derive(Debug, Clone)]
pub struct StrPathPredicate<P: predicates_core::Predicate<str>>(
    predicates::path::FileContentPredicate<
        predicates::str::Utf8Predicate<P>,
    >,
);

impl<P> StrPathPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    pub(crate) fn new(value: P) -> Self {
        let pred = value.from_utf8().from_file_path();
        StrPathPredicate(pred)
    }
}

impl<P> predicates_core::reflection::PredicateReflection for StrPathPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl<P> predicates_core::Predicate<path::Path> for StrPathPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    fn eval(&self, item: &path::Path) -> bool {
        self.0.eval(item)
    }
}

impl<P> fmt::Display for StrPathPredicate<P>
where
    P: predicates_core::Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<P> IntoPathPredicate<StrPathPredicate<P>> for P
where
    P: predicates_core::Predicate<str>,
{
    type Predicate = StrPathPredicate<P>;

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
        P: predicates_core::Predicate<path::Path>,
    {
        pred.into_path()
    }

    #[test]
    fn into_path_from_pred() {
        let pred = convert_path(predicate::eq(path::Path::new("hello.md")));
        assert!(pred.eval(path::Path::new("hello.md")));
    }

    #[test]
    fn into_path_from_bytes() {
        let pred = convert_path(b"hello\n" as &[u8]);
        assert!(pred.eval(path::Path::new("tests/fixture/hello.txt")));
    }

    #[test]
    fn into_path_from_str() {
        let pred = convert_path("hello\n");
        assert!(pred.eval(path::Path::new("tests/fixture/hello.txt")));
    }
}
