use super::super::model::post::RedditPost;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing<T> {
    pub kind: String,
    pub data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub modhash: String,
    pub dist: u32,
    pub before: Option<String>,
    pub after: Option<String>,
    pub children: Vec<RedditPost>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {}
