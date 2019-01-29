use std::path;

use tempfile;

use super::errors::*;

/// A directory in the filesystem that is automatically deleted when
/// it goes out of scope.
///
/// The [`TempDir`] type creates a directory on the file system that
/// is deleted once it goes out of scope. At construction, the
/// `TempDir` creates a new directory with a randomly generated name.
///
/// The constructor, [`TempDir::new()`], creates directories in
/// the location returned by [`std::env::temp_dir()`].
///
/// After creating a `TempDir`, work with the file system by doing
/// standard [`std::fs`] file system operations on its [`Path`],
/// which can be retrieved with [`TempDir::path()`]. Once the `TempDir`
/// value is dropped, the directory at the path will be deleted, along
/// with any files and directories it contains. It is your responsibility
/// to ensure that no further file system operations are attempted
/// inside the temporary directory once it has been deleted.
///
/// # Resource Leaking
///
/// Various platform-specific conditions may cause `TempDir` to fail
/// to delete the underlying directory. It's important to ensure that
/// handles (like [`File`] and [`ReadDir`]) to files inside the
/// directory are dropped before the `TempDir` goes out of scope. The
/// `TempDir` destructor will silently ignore any errors in deleting
/// the directory; to instead handle errors call [`TempDir::close()`].
///
/// Note that if the program exits before the `TempDir` destructor is
/// run, such as via [`std::process::exit()`], by segfaulting, or by
/// receiving a signal like `SIGINT`, then the temporary directory
/// will not be deleted.
///
/// # Examples
///
/// Create a temporary directory with a generated name:
///
/// ```
/// use assert_fs::fixture::TempDir;
///
/// let tmp_dir = TempDir::new().unwrap();
///
/// // Ensure deletion happens.
/// tmp_dir.close().unwrap();
/// ```
///
/// [`File`]: http://doc.rust-lang.org/std/fs/struct.File.html
/// [`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
/// [`ReadDir`]: http://doc.rust-lang.org/std/fs/struct.ReadDir.html
/// [`TempDir::close()`]: struct.TempDir.html#method.close
/// [`TempDir::new()`]: struct.TempDir.html#method.new
/// [`TempDir::path()`]: struct.TempDir.html#method.path
/// [`TempDir`]: struct.TempDir.html
/// [`std::env::temp_dir()`]: https://doc.rust-lang.org/std/env/fn.temp_dir.html
/// [`std::fs`]: http://doc.rust-lang.org/std/fs/index.html
/// [`std::process::exit()`]: http://doc.rust-lang.org/std/process/fn.exit.html
pub struct TempDir {
    temp: tempfile::TempDir,
}

impl TempDir {
    /// Attempts to make a temporary directory inside of `env::temp_dir()`.
    ///
    /// The directory and everything inside it will be automatically deleted
    /// once the returned `TempDir` is destroyed.
    ///
    /// # Errors
    ///
    /// If the directory can not be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::TempDir;
    ///
    /// let tmp_dir = TempDir::new().unwrap();
    ///
    /// // Ensure deletion happens.
    /// tmp_dir.close().unwrap();
    /// ```
    pub fn new() -> Result<Self, FixtureError> {
        let temp = tempfile::TempDir::new()
            .chain(FixtureError::new(FixtureKind::CreateDir))?;
        Ok(Self { temp })
    }

    /// Accesses the [`Path`] to the temporary directory.
    ///
    /// [`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::TempDir;
    ///
    /// let tmp_dir = TempDir::new().unwrap();
    ///
    /// println!("{}", tmp_dir.path().display());
    ///
    /// // Ensure deletion happens.
    /// tmp_dir.close().unwrap();
    /// ```
    pub fn path(&self) -> &path::Path {
        self.temp.path()
    }

    /// Closes and removes the temporary directory, returing a `Result`.
    ///
    /// Although `TempDir` removes the directory on drop, in the destructor
    /// any errors are ignored. To detect errors cleaning up the temporary
    /// directory, call `close` instead.
    ///
    /// # Errors
    ///
    /// This function may return a variety of [`std::io::Error`]s that result from deleting
    /// the files and directories contained with the temporary directory,
    /// as well as from deleting the temporary directory itself. These errors
    /// may be platform specific.
    ///
    /// [`std::io::Error`]: http://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    ///
    /// ```
    /// use assert_fs::fixture::TempDir;
    ///
    /// let tmp_dir = TempDir::new().unwrap();
    ///
    /// // Ensure deletion happens.
    /// tmp_dir.close().unwrap();
    /// ```
    pub fn close(self) -> Result<(), FixtureError> {
        self.temp.close()
            .chain(FixtureError::new(FixtureKind::Cleanup))?;
        Ok(())
    }
}
