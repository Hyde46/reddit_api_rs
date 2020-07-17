use super::super::model::post::RedditPost;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    kind: String,
    data: ListingData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListingData {
    modhash: String,
    dist: u32,
    before: Option<String>,
    after: Option<String>,
    children: Vec<RedditPost>,
}
