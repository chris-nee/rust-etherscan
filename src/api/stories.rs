use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

pub async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}item/{}.json", constants::BASE_API_URL, id);
    reqwest::get(&url).await?.json().await
}

pub async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", constants::BASE_API_URL);
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));
    Ok(join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect())
}
