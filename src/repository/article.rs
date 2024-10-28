use chrono::Utc;
use sqlx::PgPool;

use crate::domain::Article;
use crate::domain::NewsSource;

use anyhow::Result;

#[tracing::instrument(name = "Read articles from DB", skip(db, news_source))]
pub async fn get_by_source(db: &PgPool, news_source: NewsSource) -> Result<Vec<Article>> {
    let source: String = news_source.clone().into();
    let records = sqlx::query!(
        r#"
        SELECT id, link, title, description, tags
        FROM articles
        WHERE source = $1"#,
        source,
    )
    .fetch_all(db)
    .await?;

    let articles = records
        .into_iter()
        .map(|row| Article {
            id: row.id,
            link: row.link,
            title: row.title,
            description: row.description,
            tags: row.tags,
            source: news_source.clone(),
        })
        .collect();

    Ok(articles)
}

#[tracing::instrument(name = "Write scraped articles", skip(db, articles))]
pub async fn save(db: &PgPool, articles: Vec<Article>) -> Result<(), sqlx::Error> {
    let mut transaction = db.begin().await?;

    for article in articles {
        sqlx::query!(
            r#"
            INSERT INTO articles (id, source, title, link, description, tags, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            article.id,
            Into::<String>::into(article.source),
            article.title,
            article.link,
            article.description,
            article.tags.as_deref(),
            Utc::now(),
        )
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await.map_err(|e| {
        tracing::error!("Failed to write articles {:?}", e);
        e
    })?;

    Ok(())
}
