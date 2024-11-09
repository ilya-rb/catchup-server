use crate::domain::{Article, NewsSource, NewsSourceKind, Tag, Tags};
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

#[tracing::instrument(name = "Fetch hacker news articles", skip(http_client))]
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
        .filter_map(|item| {
            let url = Url::parse(&item.url).ok()?;
            let tags = Tags(
                item.tags
                    .iter()
                    .filter_map(|tag| Tag::new(tag.clone()).ok())
                    .collect(),
            );

            Article::new(
                item.title,
                Some(item.url.clone()),
                url,
                NewsSource::of_kind(HackerNews),
                tags,
            )
            .map_err(|e| tracing::error!("Failed to parse article {:?}", e))
            .ok()
        })
        .collect();

    Ok(articles)
}
