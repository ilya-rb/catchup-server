use anyhow::{bail, Result};

#[derive(Debug, PartialEq)]
pub struct Tag(pub String);

#[derive(Debug, PartialEq)]
pub struct Tags(pub Vec<Tag>);

impl From<Vec<String>> for Tags {
    fn from(value: Vec<String>) -> Self {
        Tags(value.into_iter().map(Tag).collect())
    }
}

impl From<Tags> for Vec<String> {
    fn from(value: Tags) -> Self {
        value.0.into_iter().map(|t| t.0).collect()
    }
}

impl Tag {
    pub fn new(value: String) -> Result<Tag> {
        if value.is_empty() {
            bail!("Tag cannot be empty");
        }
        Ok(Tag(value))
    }
}
