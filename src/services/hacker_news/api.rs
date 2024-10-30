use crate::domain::{Article, NewsSource, NewsSourceKind};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use url::Url;
use NewsSourceKind::HackerNews;

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

// TODO: Move to config
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
                // TODO: Handle error, log and skip broken articles
                Url::parse(item.url.clone().as_str()).expect("Invalid url"),
                NewsSource::of_kind(HackerNews),
                item.tags.into(),
            )
            .unwrap() // TODO: Handle error and skip broken articles
        })
        .collect();

    Ok(articles)
}
