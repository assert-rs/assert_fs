use std::ffi;
use std::path;

use tempfile;

use super::errors::*;

/// A potential file in the filesystem that is automatically deleted when
/// it goes out of scope.
///
/// The [`NamedTempFile`] type creates a directory on the file system that
/// is deleted once it goes out of scope. At construction, the
/// `NamedTempFile` creates a new directory with a randomly generated name.
///
/// The constructor, [`NamedTempFile::new(name)`], creates directories in
/// the location returned by [`std::env::temp_dir()`].
///
/// After creating a `NamedTempFile`, work with the file system by doing
/// standard [`std::fs`] file system operations on its [`Path`],
/// which can be retrieved with [`NamedTempFile::path()`]. Once the `NamedTempFile`
/// value is dropped, the parent directory will be deleted, along with the file. It is your
/// responsibility to ensure that no further file system operations are attempted inside the
/// temporary directory once it has been deleted.
///
/// # Resource Leaking
///
/// Various platform-specific conditions may cause `NamedTempFile` to fail
/// to delete the underlying directory. It's important to ensure that
/// handles (like [`File`] and [`ReadDir`]) to the file inside the
/// directory is dropped before the `NamedTempFile` goes out of scope. The
/// `NamedTempFile` destructor will silently ignore any errors in deleting
/// the directory; to instead handle errors call [`NamedTempFile::close()`].
///
/// Note that if the program exits before the `NamedTempFile` destructor is
/// run, such as via [`std::process::exit()`], by segfaulting, or by
/// receiving a signal like `SIGINT`, then the temporary directory
/// will not be deleted.
///
/// # Examples
///
/// Create a temporary file.
///
/// ```
/// use assert_fs::fixture::NamedTempFile;
///
/// let tmp_file = NamedTempFile::new("foo.rs").unwrap();
///
/// // Ensure deletion happens.
/// tmp_file.close().unwrap();
/// ```
///
/// [`File`]: http://doc.rust-lang.org/std/fs/struct.File.html
/// [`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
/// [`ReadDir`]: http://doc.rust-lang.org/std/fs/struct.ReadDir.html
/// [`NamedTempFile::close()`]: struct.NamedTempFile.html#method.close
/// [`NamedTempFile::new()`]: struct.NamedTempFile.html#method.new
/// [`NamedTempFile::path()`]: struct.NamedTempFile.html#method.path
/// [`NamedTempFile`]: struct.NamedTempFile.html
/// [`std::env::temp_dir()`]: https://doc.rust-lang.org/std/env/fn.temp_dir.html
/// [`std::fs`]: http://doc.rust-lang.org/std/fs/index.html
/// [`std::process::exit()`]: http://doc.rust-lang.org/std/process/fn.exit.html
pub struct NamedTempFile {
    temp: tempfile::TempDir,
    path: path::PathBuf,
}

impl NamedTempFile {
    /// Attempts to make a temporary file inside of `env::temp_dir()`.
    ///
    /// The file and parent directory will be automatically deleted once the returned
    /// `NamedTempFile` is destroyed.
    ///
    /// # Errors
    ///
    /// If the parent directory can not be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::NamedTempFile;
    ///
    /// let tmp_file = NamedTempFile::new("foo.rs").unwrap();
    ///
    /// // Ensure deletion happens.
    /// tmp_file.close().unwrap();
    /// ```
    pub fn new<S>(name: S) -> Result<Self, FixtureError>
    where
        S: AsRef<ffi::OsStr>
    {
        let temp = tempfile::TempDir::new()
            .chain(FixtureError::new(FixtureKind::CreateDir))?;
        let path = temp.path().join(name.as_ref());
        Ok(Self { temp, path })
    }

    /// Accesses the [`Path`] to the temporary file.
    ///
    /// [`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::NamedTempFile;
    ///
    /// let tmp_file = NamedTempFile::new("foo.rs").unwrap();
    ///
    /// println!("{}", tmp_file.path().display());
    ///
    /// // Ensure deletion happens.
    /// tmp_file.close().unwrap();
    /// ```
    pub fn path(&self) -> &path::Path {
        &self.path
    }

    /// Closes and removes the temporary file and parent directory, returing a `Result`.
    ///
    /// Although `NamedTempFile` removes the directory on drop, in the destructor
    /// any errors are ignored. To detect errors cleaning up the temporary
    /// directory, call `close` instead.
    ///
    /// # Errors
    ///
    /// This function may return a variety of [`std::io::Error`]s that result from deleting the
    /// temporary file and parent directory, These errors may be platform specific.
    ///
    /// [`std::io::Error`]: http://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::NamedTempFile;
    ///
    /// let tmp_file = NamedTempFile::new("foo.rs").unwrap();
    ///
    /// // Ensure deletion happens.
    /// tmp_file.close().unwrap();
    /// ```
    pub fn close(self) -> Result<(), FixtureError> {
        self.temp.close()
            .chain(FixtureError::new(FixtureKind::Cleanup))?;
        Ok(())
    }
}
