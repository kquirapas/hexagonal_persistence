//! Hexagonal Architecture Adapters for Persistence
use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::common::Book;

pub mod pg;

pub trait PersistenceAdapter {
    async fn create_book<'a>(&self, book: &'a Book) -> Result<()>;
}
