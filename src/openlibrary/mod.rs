use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchResults {
    pub num_found: i32,
    pub offset: Option<i32>,
    pub docs: Vec<SearchResult>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub key: String,
    pub title: String,
    pub first_publish_year: Option<i32>,
    pub isbn: Option<Vec<String>>,
    pub author_key: Option<Vec<String>>,
    pub author_name: Option<Vec<String>>,
    pub number_of_pages_median: Option<i32>,
    pub cover_i: Option<i64>,
}

pub struct OpenLibrary;

impl OpenLibrary {
    const SEARCH_URL: &'static str = "https://openlibrary.org/search.json";

    pub async fn search_for_books(
        title: Option<&str>,
        author: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        let params = [
            ("title", title.unwrap_or("\"\"")),
            ("author", author.unwrap_or("\"\"")),
        ];
        let request_url = Url::parse_with_params(Self::SEARCH_URL, &params)?;

        let response = reqwest::get(request_url).await?;
        let results: SearchResults = response.json().await?;
        Ok(results.docs)
    }
}
