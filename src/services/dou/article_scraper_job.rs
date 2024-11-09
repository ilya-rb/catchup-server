use crate::configuration::Settings;
use crate::repository;
use crate::services::dou;
use actix_web::web;
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use sqlx::PgPool;

#[tracing::instrument(name = "Run Dou scraper", skip(db, http_client, settings))]
pub async fn run_scraper(
    db: web::Data<PgPool>,
    http_client: web::Data<Client>,
    settings: web::Data<Settings>,
) -> Result<()> {
    let articles = dou::article_scraper::scrape_latest_articles(
        &http_client,
        settings.services.dou.url.clone(),
    )
    .await
    .map_err(|_| anyhow!("Failed to fetch articles"))?;

    repository::article::save(&db, articles)
        .await
        .context("Failed to save articles into database")?;

    Ok(())
}
