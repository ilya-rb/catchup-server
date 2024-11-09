use crate::domain::{Article, NewsSource, NewsSourceKind};
use crate::error::error_chain_fmt;
use crate::services::{dou, hacker_news, irish_times};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Formatter;

#[derive(Deserialize)]
pub struct QueryData {
    source: String,
}

#[derive(Serialize)]
pub struct Response {
    articles: Vec<Article>,
}

#[derive(thiserror::Error)]
pub enum NewsError {
    #[error("{0}")]
    UnsupportedSource(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Get news", skip(query, db))]
pub async fn get_news(
    query: web::Query<QueryData>,
    db: web::Data<PgPool>,
    http_client: web::Data<Client>,
) -> Result<HttpResponse, NewsError> {
    let source = NewsSource::from_key(query.source.as_str())?;
    let articles = match source.kind {
        NewsSourceKind::IrishTimes => irish_times::api::get_latest_news(source, &db).await,
        NewsSourceKind::HackerNews => hacker_news::api::get_latest_news(&http_client).await,
        NewsSourceKind::Dou => dou::api::get_latest_news(&http_client).await,
    }?;

    Ok(HttpResponse::Ok().json(web::Json(Response { articles })))
}

impl std::fmt::Debug for NewsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NewsError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewsError::UnsupportedSource(_) => StatusCode::BAD_REQUEST,
            NewsError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
