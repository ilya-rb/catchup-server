use anyhow::Result;
use reqwest::Client;
use crate::domain::Article;
use crate::services::dou::article_scraper;

pub async fn get_latest_news(http_client: &Client) -> Result<Vec<Article>> {
    let articles = article_scraper::scrape_latest_articles(
        http_client,
        "https://dou.ua/lenta".into(),
    ).await.unwrap();

    Ok(articles)
}
