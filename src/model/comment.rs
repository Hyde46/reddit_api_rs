use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedditComment {
    pub render_id: String,
    pub link_id: String,
    pub id: String,
    pub gilded: String,
    pub author: String,
    pub parent_id: String,
    pub score: String,
    pub author_fullname: String,
    pub subreddit_id: String,
    pub body: String,
    pub edited: String,
    pub stickied: String,
    pub score_hidden: String,
    pub permalink: String,
    pub distinguished: String,
    pub subreddit_name_prefixed: String,
}
