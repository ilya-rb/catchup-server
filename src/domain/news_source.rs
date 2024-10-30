use anyhow::{bail, Result};

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct NewsSource {
    pub key: String,
    pub kind: NewsSourceKind,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum NewsSourceKind {
    IrishTimes,
    HackerNews,
    Dou,
}

const KEY_IRISH_TIMES: &str = "irishtimes";
const KEY_HACKER_NEWS: &str = "hackernews";
const KEY_DOU: &str = "dou";

impl NewsSource {
    pub fn from_key(key: &str) -> Result<NewsSource> {
        let kind = match key {
            KEY_IRISH_TIMES => NewsSourceKind::IrishTimes,
            KEY_HACKER_NEWS => NewsSourceKind::HackerNews,
            KEY_DOU => NewsSourceKind::Dou,
            _ => bail!(format!("Unsupported news source kind {}", key)),
        };

        Ok(NewsSource {
            key: String::from(key),
            kind,
        })
    }

    pub fn of_kind(kind: NewsSourceKind) -> NewsSource {
        let key = match kind {
            NewsSourceKind::IrishTimes => KEY_IRISH_TIMES,
            NewsSourceKind::HackerNews => KEY_HACKER_NEWS,
            NewsSourceKind::Dou => KEY_DOU,
        };

        NewsSource {
            key: String::from(key),
            kind,
        }
    }
}
