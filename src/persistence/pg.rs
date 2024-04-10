use super::*;

pub struct PostgresPersistence {
    pub(super) pool: PgPool,
}

impl PostgresPersistence {
    pub async fn try_new() -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://superuser:superpassword@0.0.0.0:5432/postgres")
                .await
                .with_context(|| "Failed to connect to Postgres DB")?,
        })
    }
}

impl PersistenceAdapter for PostgresPersistence {
    async fn create_book(&self, book: &Book) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)",
            &book.title,
            &book.author,
            &book.isbn
        )
        .execute(&mut *transaction)
        .await
        .with_context(|| "Failed to create book")?;

        transaction.commit().await?;

        Ok(())
    }
}
