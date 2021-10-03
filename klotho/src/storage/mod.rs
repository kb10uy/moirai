//! File storage abstraction layer.

pub mod local;

pub use local::LocalFilesystem;

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

use async_trait::async_trait;

/// Type alias for `Result<T, storage::Error`.
pub type Result<T> = StdResult<T, Error>;

/// Represents an error about storage manipulation.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Option<Box<dyn 'static + StdError + Send + Sync>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Invalid configuration state was detected.
    InvalidConfiguration,

    /// Storage backend is out of storage.
    OutOfSpace,

    /// Failed to write data to storage.
    CannotWrite,

    /// File not found.
    NotFound,

    /// Other error.
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ErrorKind::InvalidConfiguration => write!(f, "Invalid storage configuration")?,
            ErrorKind::OutOfSpace => write!(f, "Storage is out of space")?,
            ErrorKind::CannotWrite => write!(f, "Failed to write to storage")?,
            ErrorKind::NotFound => write!(f, "File not found")?,
            ErrorKind::Other => write!(f, "Other storage error")?,
            // _ => write!(f, "Unknown storage error")?,
        }
        if let Some(inner) = &self.inner {
            write!(f, ": {}", inner)?;
        }
        Ok(())
    }
}

impl StdError for Error {}

impl Error {
    /// Constructs an error.
    fn new(kind: ErrorKind, inner: Option<Box<dyn 'static + StdError + Send + Sync>>) -> Error {
        Error { kind, inner }
    }

    /// Gets the error kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Gets the inner error if exists.
    pub fn inner(&self) -> Option<&(dyn StdError + Send + Sync)> {
        self.inner.as_deref()
    }
}

/// Storage manipulation abstraction.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Stores data as a file.
    /// If succeeded, object key (filename) will be returned.
    async fn store(&self, data: &[u8], extension: Option<&str>) -> Result<String>;

    /// Constructs fully qualified filepath.
    async fn path(&self, filename: &str) -> Result<String>;

    /// Drops a file with key.
    async fn remove(&self, filename: &str) -> Result<()>;
}
