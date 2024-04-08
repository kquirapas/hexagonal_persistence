//! Hexagonal Architecture Adapters for Persistence
use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

use crate::common::Book;

pub mod pg;
use self::pg::*;

pub trait PersistenceAdapter {
    async fn create_book(&self, book: Book) -> Result<Book>;
    // fn read_book() -> Result<Book>;
    // fn update_book() -> Result<Book>;
    // fn delete_book() -> Result<Book>;
}

impl PersistenceAdapter for PostgresPersistence {
    async fn create_book(&self, book: Book) -> Result<Book> {
        sqlx::migrate!().run(&self.pool).await?;

        let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(&book.title)
            .bind(&book.author)
            .bind(&book.isbn)
            .execute(&self.pool)
            .await
            .with_context(|| "Failed to create book")?;

        Ok(book)
    }
}
