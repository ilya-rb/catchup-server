use std::fmt::Formatter;

use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use anyhow::Context;
use reqwest::Client;
use sqlx::PgPool;

use crate::error::error_chain_fmt;
use crate::services::irish_times;

#[derive(thiserror::Error)]
pub enum ScraperError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Run articles scraper", skip(db, http_client))]
pub async fn run_scraper(
    db: web::Data<PgPool>,
    http_client: web::Data<Client>,
) -> Result<HttpResponse, ScraperError> {
    irish_times::article_scraper_job::run_scraper(db, http_client)
        .await
        .context("Failed to scrape articles")?;

    Ok(HttpResponse::Ok().finish())
}

impl std::fmt::Debug for ScraperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ScraperError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
