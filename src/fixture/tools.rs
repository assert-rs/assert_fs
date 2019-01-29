//! Initialize the filesystem to use as test fixtures.

use std::fs;
use std::io;
use std::io::Write;
use std::path;

use globwalk;

use super::errors::*;
use super::ChildPath;
use super::TempDir;

/// Create empty directories at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait PathCreateDir {
    /// Create an empty file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.child("subdir").create_dir_all().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn create_dir_all(&self) -> io::Result<()>;
}

impl PathCreateDir for ChildPath {
    fn create_dir_all(&self) -> io::Result<()> {
        create_dir_all(self.path())
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

/// Write (copy) a file to [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteFile {
    /// Write (copy) a file to [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::Path;
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_file(Path::new("Cargo.toml"))
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_file(&self, data: &path::Path) -> io::Result<()>;
}

impl FileWriteFile for ChildPath {
    fn write_file(&self, data: &path::Path) -> io::Result<()> {
        write_file(self.path(), data)
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

impl PathCopy for TempDir {
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

fn create_dir_all(path: &path::Path) -> io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
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

fn write_file(path: &path::Path, data: &path::Path) -> io::Result<()> {
    fs::copy(data, path)?;
    Ok(())
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
            fs::create_dir_all(target_path).chain(FixtureError::new(FixtureKind::CreateDir))?;
        } else if entry.file_type().is_file() {
            fs::create_dir_all(target_path.parent().expect("at least `target` exists"))
                .chain(FixtureError::new(FixtureKind::CreateDir))?;
            fs::copy(entry.path(), target_path).chain(FixtureError::new(FixtureKind::CopyFile))?;
        }
    }
    Ok(())
}
