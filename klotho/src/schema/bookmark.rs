//! Bookmark data types.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

/// Represents a stored bookmark.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, FromRow)]
pub struct Bookmark {
    id: i64,
    title: String,
    url: Option<String>,
    description: Option<String>,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
}

impl Bookmark {
    /// Unique ID.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Bookmark title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Bookmark URL if set.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Description text in CommonMark.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// The datetime when this bookmark created.
    pub fn created_at(&self) -> DateTime<FixedOffset> {
        self.created_at
    }

    /// The datetime when this bookmark updated last.
    pub fn updated_at(&self) -> DateTime<FixedOffset> {
        self.updated_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BookmarkBuilder {
    id: Option<i64>,
    title: String,
    url: Option<String>,
    description: Option<String>,
}

impl BookmarkBuilder {
    /// Creates a new builder.
    pub fn new(title: impl Into<String>) -> BookmarkBuilder {
        BookmarkBuilder {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Sets an existing ID.
    pub fn with_id(mut self, id: Option<i64>) -> BookmarkBuilder {
        self.id = id;
        self
    }

    /// Sets the URL.
    pub fn with_url(mut self, url: impl Into<String>) -> BookmarkBuilder {
        self.url = Some(url.into());
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, url: impl Into<String>) -> BookmarkBuilder {
        self.description = Some(url.into());
        self
    }
}
