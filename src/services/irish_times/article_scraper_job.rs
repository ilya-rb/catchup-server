use crate::configuration::Settings;
use crate::repository;
use crate::services::irish_times;
use anyhow::{Context, Result};
use reqwest::Client;
use sqlx::PgPool;

pub async fn run_scraper(db: &PgPool, http_client: &Client, settings: &Settings) -> Result<()> {
    let articles = irish_times::articles_scraper::scrape_latest_articles(
        http_client,
        &settings.services.irish_times.url,
    )
    .await?;

    repository::article::save(db, articles)
        .await
        .context("Failed to save articles into database")?;

    Ok(())
}
