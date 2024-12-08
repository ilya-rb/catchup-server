use crate::configuration::Settings;
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
    author: String,
    #[serde(rename = "_tags")]
    tags: Vec<String>,
}

pub async fn get_latest_news(http_client: &Client, settings: &Settings) -> Result<Vec<Article>> {
    let response = http_client
        .get(settings.services.hacker_news.url.as_ref())
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
                None,
                url,
                NewsSource::of_kind(HackerNews),
                tags,
                Some(item.author),
                None,
            )
            .map_err(|e| tracing::error!("Failed to parse article {:?}", e))
            .ok()
        })
        .collect();

    Ok(articles)
}
