use anyhow::{Context, Result};

mod common;
use common::Book;
pub mod persistence;
use persistence::*;

#[tokio::main]
async fn main() -> Result<()> {
    let db = persistence::pg::PostgresPersistence::try_new()
        .await
        .with_context(|| "Failed to create new PostgresPersistence")?;

    let book = Book {
        title: "New Book ko".to_string(),
        author: "Melissa Mesias".to_string(),
        isbn: "new-isbn".to_string(),
    };

    db.create_book(book).await?;

    Ok(())
}
