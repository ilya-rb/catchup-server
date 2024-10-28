use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq, serde::Serialize, EnumIter)]
pub enum NewsSource {
    IrishTimes,
    HackerNews,
    Dou,
}

pub const KEY_IRISH_TIMES: &str = "irishtimes";
pub const KEY_HACKER_NEWS: &str = "hackernews";
pub const KEY_DOU: &str = "dou";

impl NewsSource {
    pub fn key(&self) -> String {
        match self {
            NewsSource::IrishTimes => KEY_IRISH_TIMES.into(),
            NewsSource::HackerNews => KEY_HACKER_NEWS.into(),
            NewsSource::Dou => KEY_DOU.into(),
        }
    }

    pub fn keys() -> Vec<String> {
        NewsSource::iter().map(|n| n.into()).collect()
    }
}

impl TryFrom<&str> for NewsSource {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            KEY_IRISH_TIMES => Ok(NewsSource::IrishTimes),
            KEY_HACKER_NEWS => Ok(NewsSource::HackerNews),
            KEY_DOU => Ok(NewsSource::Dou),
            _ => Err(format!("Unsupported source {}", value)),
        }
    }
}

impl From<NewsSource> for String {
    fn from(value: NewsSource) -> Self {
        String::from(match value {
            NewsSource::IrishTimes => KEY_IRISH_TIMES,
            NewsSource::HackerNews => KEY_HACKER_NEWS,
            NewsSource::Dou => KEY_DOU,
        })
    }
}
