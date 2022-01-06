use sqlx::{sqlite::SqlitePoolOptions, Result, SqlitePool};

pub struct Db;

impl Db {
    pub async fn create_pool(connection_string: &str) -> Result<SqlitePool> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        sqlx::migrate!("./src/db/migrations").run(&pool).await?;

        Ok(pool)
    }
}
