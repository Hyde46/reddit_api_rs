use super::super::model::preview::Preview;

use std::fmt::Display;
use std::str::FromStr;

#[macro_use]
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub kind: String,
    pub data: RedditPostData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedditPostData {
    pub id: String,
    pub subreddit: String,
    pub selftext: String,
    pub url: String,
    pub permalink: String,
    pub author: String,
    pub author_fullname: String,
    pub ups: usize,
    pub downs: usize,
    pub saved: bool,
    pub gilded: usize,
    pub clicked: bool,
    pub title: String,
    pub is_self: bool,
    pub archived: bool,
    pub no_follow: bool,
    pub is_crosspostable: bool,
    pub pinned: bool,
    pub over_18: bool,
    pub is_video: bool,
    pub link_flair_richtext: Vec<LinkFlairRichtext>,
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: usize,
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    pub link_flair_text_color: String,
    pub upvote_ratio: f32,
    pub subreddit_type: String,
    pub total_awards_received: usize,
    pub link_flair_background_color: String,
    pub created: f64,
    pub domain: String,
    pub allow_live_comments: bool,
    pub link_flair_type: String,
    pub wls: usize,
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    pub is_robot_indexable: bool,
    pub is_reddit_media_domain: bool,
    pub is_meta: bool,
    pub media_only: bool,
    pub num_comments: usize,
    pub send_replies: bool,
    pub whitelist_status: String,
    pub subreddit_id: String,
    pub contest_mode: bool,
    pub mod_reports: Vec<String>,
    pub author_patreon_flair: bool,
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub subreddit_subscribers: usize,
    pub created_utc: f64,
    pub is_original_content: bool,
    pub num_crossposts: usize,
    pub thumbnail_width: Option<usize>,
    pub author_flair_template_id: Option<String>,
    pub user_reports: Vec<String>,
    pub post_hint: Option<String>,
    pub approved_at_utc: Option<String>,
    pub link_flair_css_class: Option<String>,
    pub thumbnail_height: Option<usize>,
    pub mod_reason_title: Option<String>,
    pub mod_note: Option<String>,
    pub banned_by: Option<String>,
    pub author_flair_type: String,
    pub category: Option<String>,
    pub selftext_html: Option<String>,
    pub likes: Option<String>,
    pub suggested_sort: Option<String>,
    pub banned_at_utc: Option<f64>,
    pub top_awarded_type: Option<String>,
    pub view_count: Option<usize>,
    pub awarders: Vec<String>,
    pub link_flair_template_id: Option<String>,
    pub author_flair_text: Option<String>,
    pub treatment_tags: Vec<String>,
    pub removed_by: Option<String>,
    pub num_reports: Option<usize>,
    pub distinguished: Option<String>,
    pub mod_reason_by: Option<String>,
    pub removal_reason: Option<String>,
    pub report_reasons: Option<String>,
    pub author_flair_background_color: Option<String>,
    pub discussion_type: Option<String>,
    pub author_flair_text_color: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub content_categories: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub removed_by_category: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub preview: Preview,
    #[serde(skip_deserializing, skip_serializing)]
    pub all_awardings: Vec<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub gildings: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub media: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub media_metadata: Vec<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub secure_media: Option<String>, 
    #[serde(skip_deserializing, skip_serializing)]
    pub media_embed: Vec<String>,
}
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkFlairRichtext {
    pub e: String,
    pub t: String,
}
