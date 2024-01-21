use std::path;

/// Access paths within [`TempDir`][crate::TempDir] for testing.
///
/// See [`ChildPath`] trait implementations.
///
/// ```rust
/// use assert_fs::prelude::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
/// temp.close().unwrap();
/// ```
///
pub trait PathChild {
    /// Access a path within the temp directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// println!("{}", temp.path().display());
    /// println!("{}", temp.child("foo/bar.txt").path().display());
    /// temp.close().unwrap();
    /// ```
    fn child<P>(&self, path: P) -> ChildPath
    where
        P: AsRef<path::Path>;
}

impl PathChild for super::TempDir {
    fn child<P>(&self, path: P) -> ChildPath
    where
        P: AsRef<path::Path>,
    {
        ChildPath::new(self.path().join(path.as_ref()))
    }
}

impl PathChild for ChildPath {
    fn child<P>(&self, path: P) -> ChildPath
    where
        P: AsRef<path::Path>,
    {
        ChildPath::new(self.path().join(path.as_ref()))
    }
}

/// A path within a [`TempDir`][crate::TempDir]
///
/// See Trait Implementations.
///
/// # Examples
///
/// ```rust
/// use assert_fs::prelude::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
///
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// temp.child("bar.txt").touch().unwrap();
///
/// temp.close().unwrap();
/// ```
///
pub struct ChildPath {
    path: path::PathBuf,
}

impl ChildPath {
    /// Wrap a path for use with extension traits.
    ///
    /// See trait implementations or [`PathChild`] for more details.
    ///
    pub fn new<P>(path: P) -> Self
    where
        P: Into<path::PathBuf>,
    {
        Self { path: path.into() }
    }

    /// Access the path.
    pub fn path(&self) -> &path::Path {
        &self.path
    }
}

impl AsRef<path::Path> for ChildPath {
    fn as_ref(&self) -> &path::Path {
        &self.path
    }
}

impl std::ops::Deref for ChildPath {
    type Target = path::Path;
    #[inline]
    fn deref(&self) -> &path::Path {
        &self.path
    }
}

/// Access existing paths within [`TempDir`][crate::TempDir] for testing
///
/// See [`ChildPath`] trait implementations.
///
/// ```rust
/// use assert_fs::prelude::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.existing_child("foo.txt").unwrap();
/// input_file.assert(predicates::path::exists());
/// temp.close().unwrap();
/// ```
///
pub trait PathExistingChild {
    /// Access an existing path within the temp directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// let child = temp.existing_child("foo").unwrap();
    /// for entry in temp.read_dir().unwrap() {
    ///     println!("{}", entry.unwrap().path().display());
    /// }
    /// temp.close().unwrap();
    /// ```
    fn existing_child<P>(&self, path: P) -> Result<ChildPath, super::FixtureError>
    where
        P: AsRef<path::Path>;
}

impl PathExistingChild for super::TempDir {
    fn existing_child<P>(&self, path: P) -> Result<ChildPath, super::FixtureError>
    where
        P: AsRef<path::Path>,
    {
        let child = self.child(path);
        super::FileTouch::touch(&child)?;
        Ok(child)
    }
}

impl PathExistingChild for ChildPath {
    fn existing_child<P>(&self, path: P) -> Result<ChildPath, super::FixtureError>
    where
        P: AsRef<path::Path>,
    {
        let child = self.child(path);
        super::FileTouch::touch(&child)?;
        Ok(child)
    }
}
