use crate::configuration::Settings;
use crate::domain::{Article, NewsSource, NewsSourceKind, Tags};
use anyhow::Result;
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    by: String,
    id: u32,
    title: String,
    url: Url,
    time: u32,
}

const ITEMS_PER_PAGE: usize = 10;
const PATH_TOP_STORIES: &str = "/topstories.json";

pub async fn get_latest_news(settings: &Settings) -> Result<Vec<Article>> {
    let base_url = settings.services.hacker_news.url.clone();
    let base_url = String::from(base_url.as_str());

    let stories_url = format!("{}/{}", base_url.clone(), PATH_TOP_STORIES);
    let firebase = Firebase::new(stories_url.as_str())?;
    let ids: Vec<u32> = firebase
        .get::<Vec<u32>>()
        .await?
        .into_iter()
        .take(ITEMS_PER_PAGE)
        .collect();

    let mut tasks = JoinSet::new();

    for id in ids {
        let base_url = base_url.clone();

        tasks.spawn(async move {
            let url = format!("{}/item/{}.json", base_url, id);

            match Firebase::new(url.as_str()) {
                Ok(firebase) => firebase.get::<Item>().await.ok(),
                Err(_) => None,
            }
        });
    }

    let items = tasks.join_all().await;
    let items: Vec<Article> = items
        .into_iter()
        .flatten()
        .filter_map(|item| {
            Article::new(
                item.title,
                None,
                item.url,
                NewsSource::of_kind(NewsSourceKind::HackerNews),
                Tags(vec![]),
                Some(item.by),
                None,
            )
            .map_err(|e| {
                tracing::error!("Error creating new article: {}", e);
                e
            })
            .ok()
        })
        .collect();

    Ok(items)
}
