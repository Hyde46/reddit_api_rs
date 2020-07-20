use super::super::model::post::RedditPost;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub kind: String,
    pub data: ListingData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListingData {
    pub modhash: String,
    pub dist: u32,
    pub before: Option<String>,
    pub after: Option<String>,
    pub children: Vec<RedditPost>,
}
