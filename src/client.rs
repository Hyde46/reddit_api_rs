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
use super::model::sort_time::SortTime;
use super::model::token::OAuthToken;
use super::oauth2::OAuthState;
use super::oauth2::RedditApiScope;
use super::oauth2::RedditClientCredentials;
use super::oauth2::RedditOAuth;
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
            basic_prefix: "https://www.reddit.com/".to_owned(),
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
    // `read` SCOPE
    //

    /// Get `/top` posts for the authenticated user
    /// `bearer_token` needs to be set for `Reddit` struct.
    /// `read` scope is required
    /// # Arguments
    ///
    /// * `t` - Filter, one of (hour, day, week, month, year, all)
    /// * `after` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `before` - fullname of a thing. Only one of `after` and `before` should be specified
    /// * `count` - a positive integer. The number of items already seen in this listing. On the html site, the builder uses this to determine when to give values for `before` and `after` in the response ( default: 0 )
    /// * `limit` - The maximum number of items desired ( maximum: 100)
    /// * `show` - filters such as "hide links that I have voted on" will be disabled.
    /// * `sr_detail` - expand subreddits
    pub fn get_top_posts(
        &self,
        t: &SortTime,
        after: &str,
        before: &str,
        count: u32,
        limit: u32,
        show: bool,
        sr_detail: bool,
    ) -> Result<(), String> {
        // Validate parameters
        if limit > 100 {
            return Err("Limit set too high. Maximum is 100".to_owned());
        }
        if after != "" && before != "" {
            return Err(
                "Set `after` XOR `before`. Do not set both to a specific value.".to_owned(),
            );
        }
        // Check if bearer token is set
        if let Some(token) = &self.bearer_token {
            // Check if correct scope is set for token
            if !token.scope.contains("read") {
                return Err("Insufficient scope rights. Need scope: `read`.".to_owned());
            } else {
                // Request top posts with set parameters
                return Ok(());
            }
        } else {
            return Err("Bearer Token not set. Authentication necessary for ".to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_default_reddit() {
        let reddit = Reddit::default();
        assert_eq!(reddit.basic_prefix, "https://www.reddit.com/".to_owned());
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
