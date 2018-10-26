//! Initialize the filesystem to use as test fixtures.

use std::fs;
use std::io;
use std::io::Write;
use std::path;

use globwalk;
use tempfile;

pub use errors::*;
pub use tempfile::TempDir;

/// Access paths within [`TempDir`] for testing.
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
/// [`TempDir`]: struct.TempDir.html
/// [`ChildPath`]: struct.ChildPath.html
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

impl PathChild for tempfile::TempDir {
    fn child<P>(&self, path: P) -> ChildPath
    where
        P: AsRef<path::Path>,
    {
        ChildPath::new(self.path().join(path.as_ref()))
    }
}

/// A path within a [`TempDir`]
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
/// [`TempDir`]: struct.TempDir.html
pub struct ChildPath {
    path: path::PathBuf,
}

impl ChildPath {
    /// Wrap a path for use with extension traits.
    ///
    /// See trait implementations or [`PathChild`] for more details.
    ///
    /// [`PathChild`]: trait.PathChild.html
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

/// Create empty files at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileTouch {
    /// Create an empty file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.child("foo.txt").touch().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn touch(&self) -> io::Result<()>;
}

impl FileTouch for ChildPath {
    fn touch(&self) -> io::Result<()> {
        touch(self.path())
    }
}

/// Write a binary file at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteBin {
    /// Write a binary file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///     .child("foo.txt")
    ///     .write_binary(b"To be or not to be...")
    ///     .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_binary(&self, data: &[u8]) -> io::Result<()>;
}

impl FileWriteBin for ChildPath {
    fn write_binary(&self, data: &[u8]) -> io::Result<()> {
        write_binary(self.path(), data)
    }
}

/// Write a text file at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteStr {
    /// Write a text file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_str("To be or not to be...")
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_str(&self, data: &str) -> io::Result<()>;
}

impl FileWriteStr for ChildPath {
    fn write_str(&self, data: &str) -> io::Result<()> {
        write_str(self.path(), data)
    }
}

/// Copy files into [`TempDir`].
///
/// [`TempDir`]: struct.TempDir.html
pub trait PathCopy {
    /// Copy files and directories into the current path from the `source` according to the glob
    /// `patterns`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.copy_from(".", &["*.rs"]).unwrap();
    /// temp.close().unwrap();
    /// ```
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>;
}

impl PathCopy for tempfile::TempDir {
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>,
    {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

impl PathCopy for ChildPath {
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>,
    {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

fn touch(path: &path::Path) -> io::Result<()> {
    fs::File::create(path)?;
    Ok(())
}

fn write_binary(path: &path::Path, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn write_str(path: &path::Path, data: &str) -> io::Result<()> {
    write_binary(path, data.as_bytes())
}

fn copy_files<S>(
    target: &path::Path,
    source: &path::Path,
    patterns: &[S],
) -> Result<(), FixtureError>
where
    S: AsRef<str>,
{
    // `walkdir`, on Windows, seems to convert "." into "" which then fails.
    let source = source
        .canonicalize()
        .chain(FixtureError::new(FixtureKind::Walk))?;
    for entry in globwalk::GlobWalkerBuilder::from_patterns(&source, patterns)
        .follow_links(true)
        .build()
        .chain(FixtureError::new(FixtureKind::Walk))?
    {
        let entry = entry.chain(FixtureError::new(FixtureKind::Walk))?;
        let rel = entry
            .path()
            .strip_prefix(&source)
            .expect("entries to be under `source`");
        let target_path = target.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(target_path)
                .chain(FixtureError::new(FixtureKind::CreateDir))?;
        } else if entry.file_type().is_file() {
            fs::create_dir_all(target_path.parent().expect("at least `target` exists"))
                .chain(FixtureError::new(FixtureKind::CreateDir))?;
            fs::copy(entry.path(), target_path)
                .chain(FixtureError::new(FixtureKind::CopyFile))?;
        }
    }
    Ok(())
}
