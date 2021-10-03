use crate::{
    database::{Error, ErrorKind, Result},
    schema::{Bookmark, BookmarkBuilder},
};

use chrono::prelude::*;
use futures::prelude::*;
use sqlx::postgres::PgPool;

/// Fetches just one bookmark.
pub async fn fetch_bookmark(pool: &PgPool, id: i64) -> Result<Bookmark> {
    let bookmark = sqlx::query_as(
        r#"
        SELECT *
        FROM "bookmarks"
        WHERE "id" = $1;
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::new(ErrorKind::Other, Some(e.into())))?
    .ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            Some(format!("Bookmark #{} not found", id).into()),
        )
    })?;

    Ok(bookmark)
}

/// Fetches bookmarks based on `created_at` column.
pub async fn fetch_bookmarks_by_created_at<'p>(
    pool: &'p PgPool,
    since: Option<DateTime<FixedOffset>>,
    until: Option<DateTime<FixedOffset>>,
    descending: bool,
) -> Result<Vec<Bookmark>> {
    let order = if descending { "DESC" } else { "ASC" };

    let where_clause = match (since, until) {
        (Some(_), Some(_)) => r#"WHERE "created_at" BETWEEN $1 AND $2"#,
        (Some(_), None) => r#"WHERE "created_at" >= $1"#,
        (None, Some(_)) => r#"WHERE "created_at" <= $2"#,
        (None, None) => {
            return Err(Error::new(
                ErrorKind::ValidationError,
                Some("Unbound range fetch is prohibited".into()),
            ));
        }
    };
    let query_string = format!(
        r#"
        SELECT *
        FROM "bookmarks"
        {}
        ORDER BY "id" {};
        "#,
        where_clause, order
    );

    let bookmarks_stream = sqlx::query_as(&query_string)
        .fetch_all(pool)
        .map_err(|e| Error::new(ErrorKind::Other, Some(e.into())))
        .await?;

    Ok(bookmarks_stream)
}

/// Registers a new bookmark.
pub async fn register_bookmark(pool: &PgPool, builder: BookmarkBuilder) -> Result<Bookmark> {
    let now = Local::now();
    let returning_bookmark = sqlx::query_as(
        r#"
        INSERT INTO "bookmarks" ("title", "url", "description", "created_at", "updated_at")
        VALUES ($1, $2, $3, $4, $4)
        RETURNING *;
        "#,
    )
    .bind(builder.title)
    .bind(builder.url)
    .bind(builder.description)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| Error::new(ErrorKind::Other, Some(e.into())))?;

    Ok(returning_bookmark)
}

/// Updates a new bookmark.
pub async fn update_bookmark(pool: &PgPool, builder: BookmarkBuilder) -> Result<Bookmark> {
    let id = match builder.id {
        Some(id) => id,
        None => {
            return Err(Error::new(
                ErrorKind::ValidationError,
                Some("No ID given for updating bookmark".into()),
            ))
        }
    };

    let now = Local::now();
    let returning_bookmark = sqlx::query_as(
        r#"
        UPDATE "bookmarks"
        SET
            "title" = $2,
            "url" = $3,
            "description" = $4,
            "updated_at" = $5
        WHERE "id" = $1
        RETURNING *;
        "#,
    )
    .bind(id)
    .bind(builder.title)
    .bind(builder.url)
    .bind(builder.description)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| Error::new(ErrorKind::Other, Some(e.into())))?;

    Ok(returning_bookmark)
}

/// Deteles a bookmark. Deleted one will be returned.
pub async fn delete_bookmark(pool: &PgPool, id: i64) -> Result<Bookmark> {
    let returning_bookmark = sqlx::query_as(
        r#"
        DELETE FROM "bookmarks"
        WHERE "id" = $1
        RETURNING *;
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::new(ErrorKind::Other, Some(e.into())))?
    .ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            Some(format!("Bookmark #{} not found", id).into()),
        )
    })?;

    Ok(returning_bookmark)
}
