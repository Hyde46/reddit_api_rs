use super::super::model::thing::Thing;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub kind: String,
    pub data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub modhash: String,
    pub dist: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub children: Vec<Child>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Child {
    pub kind: String,
    pub data: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListingCollection {
    pub listings: Vec<Listing>,
}
