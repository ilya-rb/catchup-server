use crate::domain::{Article, NewsSource, NewsSourceKind, Tags};
use anyhow::Result;
use feed_rs::parser;
use reqwest::Client;
use url::Url;
use NewsSourceKind::Dou;

pub async fn scrape_latest_articles(http_client: &Client, url: Url) -> Result<Vec<Article>> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let feed = parser::parse(body.as_bytes())?;

    let articles: Vec<Article> = feed
        .entries
        .into_iter()
        .filter_map(|entry| {
            let url = match Url::parse(entry.id.as_str()) {
                Ok(url) => url,
                Err(_) => {
                    tracing::error!("Failed to parse article url: {}", entry.id);
                    return None;
                }
            };

            let result = Article::new(
                entry.title.unwrap().content,
                entry.summary.map(|e| e.content),
                url,
                NewsSource::of_kind(Dou),
                Tags(vec![]),
            );

            result.ok()
        })
        .collect();

    Ok(articles)
}
