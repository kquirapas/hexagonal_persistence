use anyhow::{Context, Result};
use std::sync::Arc;

mod common;
use common::Book;
pub mod persistence;
use persistence::*;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Arc::new(
        persistence::pg::PostgresPersistence::try_new()
            .await
            .with_context(|| "Failed to create new PostgresPersistence")?,
    );

    let book = Book {
        title: "hotdog New Book ko".to_string(),
        author: "author's name".to_string(),
        isbn: "new-isbn-2".to_string(),
    };

    db.create_book(book).await?;

    Ok(())
}
