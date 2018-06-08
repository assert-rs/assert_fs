use std::fmt;

use failure;

pub trait ChainFail {
    fn chain<F>(self, cause: F) -> Self
    where
        F: failure::Fail;
}

pub trait ResultChainExt<T> {
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainFail;

    fn chain_with<F, C>(self, chainable: F) -> Result<T, C>
    where
        F: FnOnce() -> C,
        C: ChainFail;
}

impl<T, E> ResultChainExt<T> for Result<T, E>
where
    E: failure::Fail,
{
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainFail,
    {
        self.map_err(|e| chainable.chain(e))
    }

    fn chain_with<F, C>(self, chainable: F) -> Result<T, C>
    where
        F: FnOnce() -> C,
        C: ChainFail,
    {
        self.map_err(|e| chainable().chain(e))
    }
}

/// Fixture initialization cause.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FixtureKind {
    /// Failed when walking the source tree.
    Walk,
    /// Failed when copying a file.
    CopyFile,
    /// Failed when creating a directory.
    CreateDir,
}

impl fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FixtureKind::Walk => write!(f, "Failed when walking the source tree"),
            FixtureKind::CopyFile => write!(f, "Failed when copying a file"),
            FixtureKind::CreateDir => write!(f, "Failed when creating a directory"),
        }
    }
}

/// Failure when initializing the fixture.
#[derive(Debug)]
pub struct FixtureError {
    kind: FixtureKind,
    cause: Option<Box<failure::Fail>>,
}

impl FixtureError {
    /// Create a `FixtureError`.
    pub fn new(kind: FixtureKind) -> Self {
        Self { kind, cause: None }
    }

    /// Fixture initialization cause.
    pub fn kind(&self) -> FixtureKind {
        self.kind
    }
}

impl failure::Fail for FixtureError {
    fn cause(&self) -> Option<&failure::Fail> {
        self.cause.as_ref().map(|c| c.as_ref())
    }

    fn backtrace(&self) -> Option<&failure::Backtrace> {
        None
    }
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Some(ref cause) => write!(
                f,
                "Failed to initialize fixture: {}\nCause: {}",
                self.kind, cause
            ),
            None => write!(f, "Failed to initialize fixture: {}", self.kind),
        }
    }
}

impl ChainFail for FixtureError {
    fn chain<F>(mut self, cause: F) -> Self
    where
        F: failure::Fail,
    {
        self.cause = Some(Box::new(cause));
        self
    }
}
