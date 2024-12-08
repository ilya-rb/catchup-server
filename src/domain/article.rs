use anyhow::{bail, Result};
use serde::Serialize;
use std::cmp::max;
use url::Url;
use uuid::Uuid;

use crate::domain::tag::Tags;
use crate::domain::NewsSource;

#[derive(Debug, PartialEq, Serialize)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub short_summary: Option<String>,
    pub link: Url,
    pub source: NewsSource,
    pub tags: Tags,
    pub author_name: Option<String>,
    pub content: Option<ArticleContent>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ArticleContent {
    pub text: String,
    pub estimated_reading_time_seconds: u32,
}

impl Article {
    pub fn new(
        title: String,
        short_summary: Option<String>,
        link: Url,
        source: NewsSource,
        tags: Tags,
        author_name: Option<String>,
        content: Option<String>,
    ) -> Result<Article> {
        #[cfg(test)]
        let id = Uuid::nil();
        #[cfg(not(test))]
        let id = Uuid::new_v4();

        if title.trim().is_empty() {
            bail!("Title is empty");
        }

        if let Some(d) = short_summary.as_ref() {
            if d.trim().is_empty() {
                bail!("Short summary is empty");
            }
        }

        let content = content.map(|text| {
            let reading_time = Article::calculate_reading_time(&text);

            ArticleContent {
                text,
                estimated_reading_time_seconds: reading_time,
            }
        });

        Ok(Article {
            id,
            title,
            short_summary,
            link,
            source,
            tags,
            author_name,
            content,
        })
    }

    fn calculate_reading_time(text: &str) -> u32 {
        let average_words_per_minute: u32 = 200;
        let words: Vec<&str> = text.split_whitespace().collect();

        let words_count = words.len();
        let count: f32 = if words_count == 0 {
            0f32
        } else {
            words_count as f32 / average_words_per_minute as f32 * 60f32
        };

        max(1, count.floor() as u32)
    }
}
