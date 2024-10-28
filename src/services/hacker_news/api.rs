use crate::domain::{Article, NewsSource};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    hits: Vec<Hit>,
}

#[derive(Deserialize)]
struct Hit {
    title: String,
    url: String,
    #[serde(rename = "_tags")]
    tags: Vec<String>,
}

const URL: &str = "https://hn.algolia.com/api/v1/search?tags=front_page";

pub async fn get_latest_news(http_client: &Client) -> Result<Vec<Article>> {
    let response = http_client
        .get(URL)
        .send()
        .await?
        .json::<Response>()
        .await?;

    let articles: Vec<Article> = response
        .hits
        .into_iter()
        .map(|item| {
            Article::new(
                item.title,
                Some(item.url.clone()),
                item.url.clone(),
                NewsSource::HackerNews,
                Some(item.tags),
            )
        })
        .collect();

    Ok(articles)
}
