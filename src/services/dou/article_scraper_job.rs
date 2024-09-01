use actix_web::web;
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use sqlx::PgPool;

use crate::repository;
use crate::services::dou;

#[tracing::instrument(name = "Run Dou scraper", skip(db, http_client))]
pub async fn run_scraper(db: web::Data<PgPool>, http_client: web::Data<Client>) -> Result<()> {
    let articles =
        dou::article_scraper::scrape_latest_articles(&http_client, "https://dou.ua/lenta".into())
            .await
            .map_err(|_| anyhow!("Failed to fetch articles"))?;

    repository::article::save(&db, articles)
        .await
        .context("Failed to save articles into database")?;

    Ok(())
}
