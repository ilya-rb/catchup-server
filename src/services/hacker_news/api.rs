use crate::domain::uuid_factory::new_uuid;
use crate::domain::{Article, NewsSource};
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

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
        .map(|item| Article {
            id: new_uuid(),
            link: item.url.clone(),
            title: item.title,
            tags: Some(item.tags),
            description: Some(item.url.clone()),
            source: NewsSource::HackerNews,
        })
        .collect();

    Ok(articles)
}
