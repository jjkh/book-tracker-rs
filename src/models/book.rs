use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;

use crate::openlibrary::{OpenLibrary, SearchResult};

#[derive(Clone, sqlx::FromRow)]
pub struct Book {
    pub id: i64,
    pub title: Option<String>,
    pub author: Option<String>,
    pub book_details_id: Option<i64>,
}

impl Book {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Book>> {
        let books = sqlx::query_as!(Book, "SELECT * FROM [books]")
            .fetch_all(pool)
            .await?;

        Ok(books)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Book>> {
        let book = sqlx::query_as!(Book, "SELECT * FROM [books] WHERE [id] = ? LIMIT 1", id)
            .fetch_optional(pool)
            .await?;

        Ok(book)
    }

    pub async fn insert(pool: &SqlitePool, book: Book) -> Result<Book> {
        let new_book = sqlx::query_as!(Book, 
                "INSERT INTO [books] ([title], [author], [book_details_id]) VALUES (?, ?, ?) RETURNING *",
                book.title,
                book.author,
                book.book_details_id,
            )
            .fetch_one(pool)
            .await?;

        Ok(new_book)
    }
}

#[derive(Clone, sqlx::FromRow)]
pub struct BookDetails {
    pub id: i64,
    pub open_library_id: String,
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub author_key: Option<String>,
    pub publish_year: Option<i64>,
    pub page_count: Option<i64>,
    pub last_updated: Option<NaiveDateTime>,
}

impl From<SearchResult> for BookDetails {
    fn from(sr: SearchResult) -> Self {
        println!("from: {:?}", sr);
        Self {
            id: -1,
            open_library_id: {
                let (_, ol_id) = sr.key.rsplit_once('/').unwrap();
                ol_id.to_string()
            },
            isbn: match sr.isbn {
                Some(iv) => iv
                    .iter()
                    .find(|isbn| isbn.len() == 13)
                    .map(|isbn| isbn.to_string()),
                None => None,
            },
            title: Some(sr.title),
            author: sr.author_name.map(|an| an.first().unwrap().to_string()),
            author_key: sr.author_key.map(|ak| ak.first().unwrap().to_string()),
            publish_year: sr.first_publish_year.map(|py| py.into()),
            page_count: sr.number_of_pages_median.map(|nop| nop.into()),
            last_updated: Some(Utc::now().naive_utc()),
        }
    }
}

impl BookDetails {
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<BookDetails>> {
        let details = sqlx::query_as!(BookDetails, "SELECT * FROM [book_details] WHERE [id] = ? LIMIT 1", id)
            .fetch_optional(pool)
            .await?;

        Ok(details)
    }

    pub async fn search(
        pool: &SqlitePool,
        title: Option<&str>,
        author: Option<&str>,
    ) -> anyhow::Result<Vec<BookDetails>> {
        let results = OpenLibrary::search_for_books(title, author).await?;

        let mut book_details = Vec::<BookDetails>::new();

        for result in results {
            let details = BookDetails::upsert(pool, result.into()).await?;
            book_details.push(details);
        }

        Ok(book_details)
    }

    pub async fn upsert(pool: &SqlitePool, book_details: BookDetails) -> Result<BookDetails> {
        let details = sqlx::query_as(
            "INSERT INTO [book_details] (
                    [open_library_id], [isbn], [title], [author], [author_key], [publish_year], [last_updated], [page_count]
                ) VALUES (
                    ?, ?, ?, ?, ?, ?, ?, ?
                ) ON CONFLICT (open_library_id) DO UPDATE SET
                      isbn         = coalesce(excluded.isbn, book_details.isbn)
                    , title        = coalesce(excluded.title, book_details.title)
                    , author       = coalesce(excluded.author, book_details.author)
                    , author_key   = coalesce(excluded.author_key, book_details.author_key)
                    , publish_year = coalesce(excluded.publish_year, book_details.publish_year)
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
            .bind(book_details.publish_year)
            .bind(book_details.last_updated)
            .bind(book_details.page_count)
            .fetch_one(pool)
            .await?;

        Ok(details)
    }
}
