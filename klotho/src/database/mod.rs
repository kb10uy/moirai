//! Bookmark database manipulation and types.

mod bookmark;

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

pub use self::bookmark::{
    delete_bookmark, fetch_bookmark, fetch_bookmarks_by_created_at, register_bookmark,
    update_bookmark,
};

/// Type alias for `Result<T, bookmark::Error>`.
pub type Result<T> = StdResult<T, Error>;

/// Represents an error about database manipulation.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Option<Box<dyn 'static + StdError + Send + Sync>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Invalid configuration state was detected.
    ValidationError,

    /// Bookmark not found.
    NotFound,

    /// Other error.
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ErrorKind::ValidationError => write!(f, "Validation error")?,
            ErrorKind::NotFound => write!(f, "Bookmark not found")?,
            ErrorKind::Other => write!(f, "Other storage error")?,
            // _ => write!(f, "Unknown database error")?,
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
