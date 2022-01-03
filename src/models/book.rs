use chrono::{DateTime, Utc};
use sqlx::{Result, SqlitePool};

#[derive(Clone, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub book_details_id: Option<i32>,
}

impl Book {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Book>> {
        sqlx::query_as("SELECT * FROM [books]")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Book>> {
        sqlx::query_as("SELECT * FROM [books] WHERE [id] = ? LIMIT 1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn insert(pool: &SqlitePool, book: Book) -> Result<Book> {
        sqlx::query_as("INSERT INTO [books] ([title], [author], [book_details_id]) VALUES (?, ?, ?) RETURNING *")
            .bind(book.title)
            .bind(book.author)
            .bind(book.book_details_id)
            .fetch_one(pool)
            .await
    }
}

#[derive(Clone, sqlx::FromRow)]
pub struct BookDetails {
    pub id: i32,
    pub open_library_id: i32,
    pub isbn: Option<i32>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub author_key: Option<String>,
    pub publish_date: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    pub page_count: Option<i32>,
}

impl BookDetails {
    pub async fn get_by_id(pool: &SqlitePool, id: i32) -> Result<Option<BookDetails>> {
        sqlx::query_as("SELECT * FROM [book_details] WHERE [id] = ? LIMIT 1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn upsert(pool: &SqlitePool, book_details: BookDetails) -> Result<BookDetails> {
        sqlx::query_as(
            "INSERT INTO [book_details] (
                    [open_library_id], [isbn], [title], [author], [author_key], [publish_date], [last_updated], [page_count]
                ) VALUES (
                    ?, ?, ?, ?, ?, ?, ?, ?
                ) ON CONFLICT (open_library_id) DO UPDATE SET
                      isbn         = coalesce(excluded.isbn, book_details.isbn)
                    , title        = coalesce(excluded.title, book_details.title)
                    , author       = coalesce(excluded.author, book_details.author)
                    , author_key   = coalesce(excluded.author_key, book_details.author_key)
                    , publish_date = coalesce(excluded.publish_date, book_details.publish_date)
                    , last_updated = coalesce(excluded.last_updated, book_details.last_updated)
                    , page_count   = coalesce(excluded.page_count, book_details.page_count)
                WHERE excluded.last_updated > book_details.last_updated
                RETURNING *"
            )
            .bind(book_details.open_library_id)
            .bind(book_details.isbn)
            .bind(book_details.title)
            .bind(book_details.author)
            .bind(book_details.author_key)
            .bind(book_details.publish_date)
            .bind(book_details.last_updated)
            .bind(book_details.page_count)
            .fetch_one(pool)
            .await
    }
}
