#[derive(Debug, PartialEq)]
pub struct Tag(pub String);

#[derive(Debug, PartialEq)]
pub struct Tags(pub Vec<Tag>);

impl From<Vec<String>> for Tags {
    fn from(value: Vec<String>) -> Self {
        Tags(value.into_iter().map(|t| Tag(t)).collect())
    }
}

impl Into<Vec<String>> for Tags {
    fn into(self) -> Vec<String> {
        self.0.into_iter().map(|t| t.0).collect()
    }
}

impl Tag {
    pub fn new(value: String) -> Self {
        assert!(!value.is_empty());
        Tag(value)
    }
}
