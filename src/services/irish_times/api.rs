use anyhow::Result;
use sqlx::PgPool;

use crate::domain::{Article, NewsSource, NewsSourceKind};
use crate::repository;

pub async fn get_latest_news(news_source: NewsSource, db: &PgPool) -> Result<Vec<Article>> {
    assert_eq!(news_source.kind, NewsSourceKind::IrishTimes);
    repository::article::get_by_source(db, news_source).await
}
