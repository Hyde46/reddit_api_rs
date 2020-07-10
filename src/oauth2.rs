use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

use super::util::generate_random_string;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum OAuthState {
    IDLE,
    AUTHORIZED,
    ERROR,
}
impl std::fmt::Display for OAuthState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
/// Reddit API Scope strings. Defines the scope, a bearer token is limited to
pub enum RedditApiScope {
    identity,
    edit,
    flair,
    history,
    modconfig,
    modflair,
    modlog,
    modposts,
    modwiki,
    mysubreddits,
    privatemessages,
    read,
    report,
    save,
    submit,
    subscribe,
    vote,
    wikiedit,
    wikiread,
}
impl std::fmt::Display for RedditApiScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditOAuthClient {
    pub oauth_bearer_token: Option<OAuthToken>,
    pub callback_url: String,
    pub oauth_state: OAuthState,
    /// If OAuthClient is in error state, then displays last error
    pub error_state: Option<String>,
    /// Randomly generated string to validate reddit oauth responses
    state_string: String,
}
impl RedditOAuthClient {
    pub fn default() -> RedditOAuthClient {
        dotenv().ok();
        let callback_url = env::var("REDIRECT_URI").unwrap_or_default();
        RedditOAuthClient {
            oauth_bearer_token: None,
            callback_url: callback_url,
            oauth_state: OAuthState::IDLE,
            error_state: None,
            state_string: generate_random_string(10),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub scope: String,
    pub refresh_token: String,
}

/// Reddit Client credentials object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditClientCredentials {
    pub fn default() -> RedditClientCredentials {
        dotenv().ok();
        let client_id = env::var("CLIENT_ID").unwrap_or_default();
        let client_secret = env::var("CLIENT_SECRET").unwrap_or_default();

        trace!(
            "RedditClientCredentials.default(): client_id:{:?}, client_secret:{:?}",
            client_id,
            client_secret
        );
        RedditClientCredentials {
            client_id,
            client_secret,
        }
    }
    /// Set `client_id`
    pub fn client_id(mut self, client_id: &str) -> RedditClientCredentials {
        self.client_id = client_id.to_owned();
        self
    }
    /// Set `client_secret`
    pub fn client_secret(mut self, client_secret: &str) -> RedditClientCredentials {
        self.client_secret = client_secret.to_owned();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_credentials_default() {
        let rcc = RedditClientCredentials::default();
        assert_eq!(rcc.client_id, "7tMofTv8Ip3-Ig".to_owned());
        assert_eq!(rcc.client_secret, "N3RNb2ZUdjhJcDMtSWc6".to_owned());
    }
}
