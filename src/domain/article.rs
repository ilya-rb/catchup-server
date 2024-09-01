use serde::Serialize;
use uuid::Uuid;

use crate::domain::NewsSource;

#[derive(Debug, PartialEq, Serialize)]
pub struct Article {
    pub id: Uuid,
    pub link: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub source: NewsSource,
}
