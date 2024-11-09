use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Tag(pub String);

#[derive(Debug, PartialEq, Serialize)]
pub struct Tags(pub Vec<Tag>);

impl Tag {
    pub fn new(value: String) -> Result<Tag> {
        if value.is_empty() {
            bail!("Tag cannot be empty");
        }
        Ok(Tag(value))
    }
}

impl From<Tags> for Vec<String> {
    fn from(value: Tags) -> Self {
        value.0.into_iter().map(|t| t.0).collect()
    }
}
