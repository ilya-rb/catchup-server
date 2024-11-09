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
            let result = Article::new(
                item.title,
                Some(item.url.clone()),
                url,
                NewsSource::of_kind(HackerNews),
                item.tags.into(),
            );

            match result {
                Ok(r) => Some(r),
                Err(e) => {
                    tracing::error!("Failed to parse article {:?}", e);
                    None
                }
            }
        })
        .collect();

    Ok(articles)
}
