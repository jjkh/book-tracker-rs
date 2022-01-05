use anyhow::{bail, Result};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use self::book::{Book, BookDetails};

pub mod book;

pub struct QueryRoot;

#[Object]
impl BookDetails {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn open_library_id(&self) -> Result<&String> {
        if let Some(open_library_id) = &self.open_library_id {
            Ok(open_library_id)
        } else {
            bail!("open_library_id is None")
        }
    }
    async fn isbn(&self) -> &Option<String> {
        &self.isbn
    }
    async fn title(&self) -> &Option<String> {
        &self.title
    }
    async fn author(&self) -> &Option<String> {
        &self.author
    }
    async fn author_key(&self) -> &Option<String> {
        &self.author_key
    }
    async fn publish_year(&self) -> &Option<i32> {
        &self.publish_year
    }
    async fn last_updated(&self) -> &Option<DateTime<Utc>> {
        &self.last_updated
    }
    async fn page_count(&self) -> &Option<i32> {
        &self.page_count
    }
}

#[Object]
impl Book {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }
    async fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }
    async fn book_details(&self, ctx: &Context<'_>) -> Result<Option<BookDetails>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        return match self.book_details_id {
            Some(id) => BookDetails::get_by_id(pool, id).await,
            None => Ok(None),
        };
    }
}

#[Object]
impl QueryRoot {
    /// Returns the books in the system
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<Book>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        Book::list(pool).await
    }

    async fn book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of book")] id: i32,
    ) -> Result<Option<Book>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        Book::get_by_id(pool, id).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn find_book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "title of book")] title: Option<String>,
        #[graphql(desc = "author of book")] author: Option<String>,
    ) -> Result<Vec<BookDetails>> {
        if title.is_none() && author.is_none() {
            bail!("at least one of title or author must be supplied");
        }

        let pool = ctx.data_unchecked::<SqlitePool>();
        let details = BookDetails::search(pool, title.as_deref(), author.as_deref()).await?;

        Ok(details)
    }

    async fn add_book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "title of book")] title: Option<String>,
        #[graphql(desc = "author of book")] author: Option<String>,
        #[graphql(desc = "id of the bookDetails for this book")] book_details_id: Option<i32>,
    ) -> Result<Book> {
        let pool = ctx.data_unchecked::<SqlitePool>();

        Book::insert(
            pool,
            Book {
                id: -1,
                title,
                author,
                book_details_id,
            },
        )
        .await
    }
}
