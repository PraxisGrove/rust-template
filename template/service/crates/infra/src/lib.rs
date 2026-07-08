use std::future::Future;
use std::pin::Pin;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use {{crate_name}}_app::{ReadinessError, ReadinessProbe};

#[derive(Debug, Clone)]
pub struct PostgresReadinessProbe {
    pool: PgPool,
}

impl PostgresReadinessProbe {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ReadinessProbe for PostgresReadinessProbe {
    fn check<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), ReadinessError>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query("SELECT 1")
                .execute(&self.pool)
                .await
                .map_err(|err| ReadinessError::new(err.to_string()))?;

            Ok(())
        })
    }
}
