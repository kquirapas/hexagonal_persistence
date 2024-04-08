use super::*;

pub struct PostgresPersistence {
    pub(super) pool: PgPool,
}

impl PostgresPersistence {
    pub async fn try_new() -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(
                    "postgres://superuser:superpassword@0.0.0.0:5432/postgres", // env::var("DATABASE_URL")
                                                                                //     .with_context(|| {
                                                                                //         "Failed to get connection string from environment variable"
                                                                                //     })?
                                                                                //     .as_ref(),
                )
                .await
                .with_context(|| "Failed to connect to Postgres DB")?,
        })
    }
}
