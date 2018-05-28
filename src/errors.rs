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

/// Failure when initializing the fixture.
#[derive(Debug, Default)]
pub struct FixtureError {
    cause: Option<Box<failure::Fail>>,
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
            Some(ref cause) => write!(f, "Failed to initialize fixture: {}", cause),
            None => write!(f, "Failed to initialize fixture"),
        }
    }
}

impl ChainFail for FixtureError {
    fn chain<F>(self, cause: F) -> Self
    where
        F: failure::Fail,
    {
        Self {
            cause: Some(Box::new(cause)),
        }
    }
}
