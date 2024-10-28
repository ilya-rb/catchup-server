use anyhow::{bail, Result};
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::domain::tag::Tags;
use crate::domain::NewsSource;

#[derive(Debug, PartialEq, Serialize)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub link: Url,
    pub source: NewsSource,
    pub tags: Tags,
}

impl Article {
    pub fn new(
        title: String,
        description: Option<String>,
        link: Url,
        source: NewsSource,
        tags: Tags,
    ) -> Result<Article> {
        #[cfg(test)]
        let id = Uuid::nil();
        #[cfg(not(test))]
        let id = Uuid::new_v4();

        if title.trim().is_empty() {
            bail!("Title is empty");
        }

        if let Some(d) = description.as_ref() {
            if d.trim().is_empty() {
                bail!("Description is empty");
            }
        }

        Ok(Article {
            id,
            title,
            description,
            link,
            source,
            tags,
        })
    }
}
