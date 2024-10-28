use serde::Serialize;
use uuid::Uuid;

use crate::domain::NewsSource;

#[derive(Debug, PartialEq, Serialize)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub source: NewsSource,
    pub tags: Option<Vec<String>>,
}

impl Article {
    pub fn new(
        title: String,
        description: Option<String>,
        link: String,
        source: NewsSource,
        tags: Option<Vec<String>>,
    ) -> Article {
        #[cfg(test)]
        let id = Uuid::nil();
        #[cfg(not(test))]
        let id = Uuid::new_v4();

        Article {
            id,
            title,
            description,
            link,
            source,
            tags,
        }
    }
}
