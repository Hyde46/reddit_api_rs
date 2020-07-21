// Built in libraries
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::string::String;

// Third party libraries
use curl::easy::{Easy, List};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Own includes
use super::curl_utils::*;
use super::model::listing::Listing;
use super::model::listing::Post;
use super::model::responses::comment_response::CommentResponse;
use super::model::sort_time::SortTime;
use super::model::token::OAuthToken;
use super::oauth2::OAuthState;
use super::oauth2::RedditApiScope;
use super::oauth2::RedditClientCredentials;
use super::oauth2::RedditOAuth;
use super::util::convert_map_to_string;
use super::VERSION;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reddit API client.
/// Allows to communicate with reddit REST and oauth2 endpoints
/// See: https://www.reddit.com/dev/api/oauth/
/// Endpoints are partially implemented.
/// See github repository for current implementations and roadmap
pub struct Reddit {
    pub authorized_prefix: String,
    pub basic_prefix: String,
    pub oauth_prefix: String,
    pub client_credentials: RedditClientCredentials,
    pub bearer_token: Option<OAuthToken>,
    is_built: bool,
}

impl Reddit {
    /// Creates default Reddit client object, assuming default reddit api endpoint urls
    ///
    /// # Example
    /// ```
    /// use reddit_api::client::Reddit;
    /// let reddit = Reddit::default();
    /// ```
    /// Note: object is in usable state after `build()` is called.
    pub fn default() -> Reddit {
        Reddit {
            authorized_prefix: "https://www.reddit.com/api/v1/".to_owned(),
            basic_prefix: "https://www.reddit.com".to_owned(),
            oauth_prefix: "https://oauth.reddit.com".to_owned(),
            client_credentials: RedditClientCredentials::default(),
            bearer_token: None,
            is_built: false,
        }
    }
    /// Sets Client credentials if custom ones are wished
    ///
    /// # Example
    /// ```
    /// use reddit_api::client::Reddit;
    /// use reddit_api::oauth2::RedditClientCredentials;
    /// let credentials = RedditClientCredentials::default().client_id("ABC");
    /// let reddit = Reddit::default()
    ///                      .client_credentials(&credentials);
    /// ```
    pub fn client_credentials(mut self, client_credentials: &RedditClientCredentials) -> Reddit {
        self.client_credentials = client_credentials.clone();
        self
    }
    /// Sets reddit api oauth endpoint url
    ///
    /// # Example
    /// ```
    /// use reddit_api::client::Reddit;
    /// let reddit = Reddit::default()
    ///                      .authorized_prefix("https://www.alternate_reddit.com/api/v1/");
    /// ```
    pub fn authorized_prefix(mut self, prefix: &str) -> Reddit {
        self.authorized_prefix = prefix.to_owned();
        self
    }
    /// Sets reddit api url for non-authorized endpoints
    ///
    /// # Example
    /// ```
    /// use reddit_api::client::Reddit;
    /// let reddit = Reddit::default()
    ///                      .basic_prefix("https://www.alternate_reddit.com/");
    /// ```
    pub fn basic_prefix(mut self, prefix: &str) -> Reddit {
        self.basic_prefix = prefix.to_owned();
        self
    }
    /// Set bearer token
    pub fn bearer_token(mut self, token: OAuthToken) -> Reddit {
        self.bearer_token = Some(token.clone());
        self
    }
    /// Validates Reddit object in a basic manner.
    /// After calling, object is ready to use
    pub fn build(mut self) -> Reddit {
        if self.authorized_prefix == "" || self.basic_prefix == "" {
            panic!("No prefixes provided. Cannot communicate with reddit API endpoint!");
        }
        self.is_built = true;
        self
    }

    //
    // `submit` SCOPE
    //

    /// Submit a new comment or reply to a message
    /// # Arguments
    /// * `thing_id` fullname of partent thing being replied to. Can be fullname of a link or fullname of a comment ( requires `submit` scope ).
    /// * `text` Raw markdown text to comment
    ///
    /// TODO: add private messaging comments
    pub fn comment(&self, thing_id: &str, text: &str) -> Result<(), String> {
        // Check if bearer token is set
        if let Some(token) = &self.bearer_token {
            if !token.scope.contains("submit") {
                return Err("Insufficient scope rights. Need scope: `submit`.".to_owned());
            } else {
                let endpoint = "/api/comment";
                let url = format!("{}{}", self.oauth_prefix, endpoint);
                let data_header = format!("Authorization: bearer {}", token.access_token);
                let mut payload_map: HashMap<String, String> = HashMap::new();
                payload_map.insert("api_type".to_owned(), "json".to_owned());
                payload_map.insert("text".to_owned(), text.to_owned());
                payload_map.insert("return_rtjson".to_owned(), "true".to_owned());
                payload_map.insert("thing_id".to_owned(), thing_id.to_owned());
                let payload_data = convert_map_to_string(&payload_map);
                let answer = post(&url, &payload_data, &data_header);
                let comment_response: CommentResponse = serde_json::from_str(&answer).unwrap();
                if let Some(err) = comment_response.json {
                    //TODO: error out of potential many errors
                    let err_msg = format!("[{}] - {}", err.errors[0][0], err.errors[0][1]);
                    return Err(err_msg);
                }
                return Ok(());
            }
        } else {
            return Err("Bearer Token not set. Authorization necessary for commenting".to_owned());
        }
    }

    //
    // `read` SCOPE
    //

    /// Get `/best` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn best(
        &self,
        subreddit: Option<&str>,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "best".to_string();
        self.get_post_by_sorting(
            sorting, subreddit, None, after, before, count, limit, show, sr_detail,
        )
    }

    /// Get `/hot` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn hot(
        &self,
        subreddit: Option<&str>,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "hot".to_string();
        self.get_post_by_sorting(
            sorting, subreddit, None, after, before, count, limit, show, sr_detail,
        )
    }

    /// Get `/rising` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn rising(
        &self,
        subreddit: Option<&str>,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "rising".to_string();
        self.get_post_by_sorting(
            sorting, subreddit, None, after, before, count, limit, show, sr_detail,
        )
    }

    /// Get `/new` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn new(
        &self,
        subreddit: Option<&str>,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "new".to_string();
        self.get_post_by_sorting(
            sorting, subreddit, None, after, before, count, limit, show, sr_detail,
        )
    }

    /// Get `/top` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn top(
        &self,
        subreddit: Option<&str>,
        t: SortTime,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "top".to_string();
        self.get_post_by_sorting(
            sorting,
            subreddit,
            Some(t),
            after,
            before,
            count,
            limit,
            show,
            sr_detail,
        )
    }

    /// Get `/controversial` posts
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `subreddit` - subreddit name to fetch posts from. E.g. /r/rust. If Option not set, posts from frontpage are fetched
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    ///
    /// # Returns
    /// `Result<Listing, String>` Either Listing of posts or Error message
    pub fn controversial(
        &self,
        subreddit: Option<&str>,
        t: SortTime,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        let sorting = "controversial".to_string();
        self.get_post_by_sorting(
            sorting,
            subreddit,
            Some(t),
            after,
            before,
            count,
            limit,
            show,
            sr_detail,
        )
    }

    fn get_post_by_sorting(
        &self,
        sorting: String,
        subreddit: Option<&str>,
        t: Option<SortTime>,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<Listing<Post>, String> {
        // Validate parameters
        if limit > 100 || limit <= 0 {
            return Err("Limit bounds are [1, 100]".to_owned());
        }
        if after != "" && before != "" {
            return Err(
                "Set `after` XOR `before`. Do not set both to a specific value.".to_owned(),
            );
        }
        // Get SortTime
        let sort_time_filter = if let Some(sort_t) = t {
            sort_t.to_string()
        } else {
            "".to_string()
        };
        // Get subreddit to filter top posts from
        let subreddit_string = if let Some(sub) = subreddit { sub } else { "" };
        // Check if bearer token is set
        if let Some(token) = &self.bearer_token {
            // Check if correct scope is set for token
            if !token.scope.contains("read") {
                return Err("Insufficient scope rights. Need scope: `read`.".to_owned());
            } else {
                // Request top posts with set parameters
                // build authorization HashMap
                let mut params: HashMap<String, String> = HashMap::new();
                params.insert("t".to_owned(), sort_time_filter);
                params.insert("limit".to_owned(), limit.to_string());
                params.insert("before".to_owned(), before.to_owned());
                params.insert("after".to_owned(), after.to_owned());
                params.insert("count".to_owned(), count.to_string());
                params.insert("show".to_owned(), show.to_string());
                params.insert("sr_detail".to_owned(), sr_detail.to_string());
                params.insert("raw_json".to_owned(), "1".to_string());
                let query_string = convert_map_to_string(&params);
                let url = format!(
                    "https://www.reddit.com{}/{}/.json?{}",
                    subreddit_string, sorting, query_string
                );
                let data_header = format!(
                    "Authorization: Basic {}",
                    self.client_credentials.client_secret
                );
                let answer = get(&url, &data_header);
                let listing: Listing<Post> = serde_json::from_str(&answer).unwrap();
                return Ok(listing);
            }
        } else {
            return Err("Bearer Token not set. Authorization necessary for this action".to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_default_reddit() {
        let reddit = Reddit::default();
        assert_eq!(reddit.basic_prefix, "https://www.reddit.com".to_owned());
        assert_eq!(
            reddit.authorized_prefix,
            "https://www.reddit.com/api/v1/".to_owned()
        );
    }

    #[test]
    fn test_reddit_usable_after_build() {
        let reddit = Reddit::default().build();
        assert!(reddit.is_built);
    }

    #[test]
    #[should_panic(expected = "No prefixes provided. Cannot communicate with reddit API endpoint!")]
    fn test_use_reddit_without_oauth_prefix() {
        Reddit::default().authorized_prefix("").build();
    }
    #[test]
    #[should_panic(expected = "No prefixes provided. Cannot communicate with reddit API endpoint!")]
    fn test_use_reddit_without_basic_prefix() {
        Reddit::default().basic_prefix("").build();
    }
}
