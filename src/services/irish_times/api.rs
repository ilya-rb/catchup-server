use crate::configuration::Settings;
use crate::domain::{Article, NewsSource, NewsSourceKind};
use crate::services::irish_times::articles_scraper;
use anyhow::Result;
use reqwest::Client;

pub async fn get_latest_news(
    news_source: NewsSource,
    http_client: &Client,
    settings: &Settings,
) -> Result<Vec<Article>> {
    assert_eq!(news_source.kind, NewsSourceKind::IrishTimes);
    articles_scraper::scrape_latest_articles(http_client, &settings.services.irish_times.url).await
}
