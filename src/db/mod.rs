use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, Result, SqlitePool,
};

pub struct Db;

impl Db {
    pub async fn create_pool(connection_string: &str) -> Result<SqlitePool> {
        // create the database if it doesn't exist
        SqliteConnectOptions::from_str(connection_string)?
            .create_if_missing(true)
            .connect()
            .await?;

        // create the database pool and update the schema
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;
        sqlx::migrate!("./src/db/migrations").run(&pool).await?;

        Ok(pool)
    }
}
