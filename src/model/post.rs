use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub render_id: String,
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub ups: String,
    pub score: String,
    pub gilded: String,
    pub link_flair_text: String,
    pub author: String,
    pub permalink: String,
    pub url: String,
    pub before: String,
    pub after: String,
}
