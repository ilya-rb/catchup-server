use crate::configuration::Settings;
use crate::domain::Article;
use crate::services::dou::article_scraper;
use anyhow::Result;
use reqwest::Client;

pub async fn get_latest_news(http_client: &Client, settings: &Settings) -> Result<Vec<Article>> {
    let articles =
        article_scraper::scrape_latest_articles(http_client, settings.services.dou.url.clone())
            .await?;

    Ok(articles)
}
