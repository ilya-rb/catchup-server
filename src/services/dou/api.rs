use crate::domain::Article;
use crate::services::dou::article_scraper;
use anyhow::Result;
use reqwest::Client;
use url::Url;

pub async fn get_latest_news(http_client: &Client) -> Result<Vec<Article>> {
    let articles =
        article_scraper::scrape_latest_articles(http_client, Url::parse("https://dou.ua/lenta")?)
            .await?;

    Ok(articles)
}
