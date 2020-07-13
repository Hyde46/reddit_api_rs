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
    pub state_string: String,
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
    /// Set `state_string`
    pub fn state_string(mut self, state_string: &str) -> RedditOAuthClient {
        self.state_string = state_string.to_owned();
        self
    }
    /// Set `oauth_bearer_token`
    pub fn oauth_bearer_token(mut self, oauth_bearer_token: &OAuthToken) -> RedditOAuthClient {
        self.oauth_bearer_token = Some(oauth_bearer_token.clone());
        self
    }
    /// Set `oauth_state`
    pub fn oauth_state(mut self, oauth_state: &OAuthState) -> RedditOAuthClient {
        self.oauth_state = oauth_state.clone();
        self
    }
    /// Set `error_state'
    pub fn error_state(mut self, error_state: &str) -> RedditOAuthClient {
        self.error_state = Some(error_state.to_owned());
        self
    }

    /// Authorize user by opening default browser and present reddit authorization dialog
    pub fn authorize_client_browser_based(mut self, url: &str) {}
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
    /*
    /// Authorizes Reddit client by opening the default webbrowser, asking the user to grant access for this application to act in the name of the authorized user account /// # Arguments
    ///
    /// * `duration` - Option of Either AuthorizationTimeOption::Permanent or AuthorizationTimeOption::Temporary. If not set, defaults to AuthorizationTimeOption::Permanent
    pub fn authorize_reddit_user(
        &self,
        duration: Option<AuthorizationTimeOption>,
    ) -> Result<String, String> {
        if !self.is_built {
            return Err("Object not built. Run `build()` before calling this method.".to_string());
        }
        let authorize_endpoint = "authorize";
        // Get `duration` string if option is set
        let mut duration_string = AuthorizationTimeOption::permanent.to_string();
        if let Some(duration) = duration {
            duration_string = duration.to_string();
        }
        // build authorization HashMap
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("response_type".to_owned(), "code".to_owned());
        params.insert("duration".to_owned(), duration_string);
        params.insert("scope".to_owned(), RedditApiScope::identity.to_string());
        params.insert("state".to_owned(), self.oauth_client.state_string.clone());
        params.insert(
            "client_id".to_owned(),
            self.client_credentials.client_id.to_owned(),
        );
        params.insert(
            "redirect_uri".to_owned(),
            self.oauth_client.callback_url.clone(),
        );
        let query_string = convert_map_to_string(&params);
        let authorize_url = format!(
            "{}{}?{}",
            self.authorized_prefix, authorize_endpoint, query_string
        );
        self.oauth_client
            .authorize_client_browser_based(&authorize_url);
        return Ok("All good".to_string());
    }*/
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
