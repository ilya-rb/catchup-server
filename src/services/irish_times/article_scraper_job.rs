use crate::repository;
use crate::services::irish_times;
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use sqlx::PgPool;
use url::Url;

#[tracing::instrument(name = "Run Irish times scraper", skip(db, http_client))]
pub async fn run_scraper(db: &PgPool, http_client: &Client) -> Result<()> {
    let tag: String = "technology".into();
    let articles = irish_times::articles_scraper::scrape_latest_articles(
        http_client,
        Url::parse(format!("https://irishtimes.com/{}", tag).as_str()).unwrap(),
        tag,
    )
    .await
    .map_err(|_| anyhow!("Failed to fetch articles"))?;

    repository::article::save(db, articles)
        .await
        .context("Failed to save articles into database")?;

    Ok(())
}
