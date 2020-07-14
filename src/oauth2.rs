//Built in libraries
use std::collections::HashMap;
use std::env;
use std::fmt;

//Third party libraries
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//Own stuff
use super::callback_server::get_browser_response;
use super::util::convert_map_to_string;
use super::util::generate_random_string;
use super::util::open_browser;

#[derive(PartialEq, Debug)]
/// Determines during client authorization whether the token is permanent or temporary
pub enum AuthorizationTime {
    permanent,
    temporary,
}
impl fmt::Display for AuthorizationTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
pub struct RedditOAuth {
    pub callback_url: String,
    pub oauth_state: OAuthState,
    /// If OAuthClient is in error state, then displays last error
    pub error_string: Option<String>,
    /// Randomly generated string to validate reddit oauth responses
    pub state_string: String,
    /// Credentials for reddit_api application
    pub client_credentials: RedditClientCredentials,
}
impl RedditOAuth {
    pub fn default() -> RedditOAuth {
        dotenv().ok();
        let callback_url = env::var("REDIRECT_URI").unwrap_or_default();
        RedditOAuth {
            callback_url: callback_url,
            oauth_state: OAuthState::IDLE,
            error_string: None,
            state_string: generate_random_string(10),
            client_credentials: RedditClientCredentials::default(),
        }
    }
    /// Set `state_string`
    pub fn state_string(mut self, state_string: &str) -> RedditOAuth {
        self.state_string = state_string.to_owned();
        self
    }
    /// Set `oauth_state`
    pub fn oauth_state(mut self, oauth_state: OAuthState) -> RedditOAuth {
        self.oauth_state = oauth_state.clone();
        self
    }
    /// Set `error_string'
    pub fn error_state(mut self, error_state: &str) -> RedditOAuth {
        self.error_string = Some(error_state.to_owned());
        self
    }
    /// Validate RedditOAuth object
    /// After calling, object is ready to use
    pub fn build(mut self) -> RedditOAuth {
        let error_flag = self.callback_url == "" || self.state_string == "";
        if error_flag {
            self.oauth_state = OAuthState::ERROR;
            self.error_string = Some("`callback_url` and `state_string` have to be set".to_owned());
            return self;
        }
        self.oauth_state = OAuthState::IDLE;
        self
    }

    /// Authorize user by opening default browser and present reddit authorization dialog
    /// to receive Bearer Token
    ///
    /// # Arguments
    ///
    /// * `scope` - String of concatenated scopes the bearer token should have authorization of
    /// * `duration` - AuthorizationTime::permanent or AuthorizationTime::temporary
    ///
    /// # Returns
    /// `Option<OAuthToken>` If authorization was successfull, OAuthToken is set, otherwise `RedditOAuth.oauth_state=OAuthState::error` with an error message in `RedditOAuth.error_string`
    pub fn authorize_client(
        mut self,
        scope: &str,
        duration: Option<AuthorizationTime>,
    ) -> Option<OAuthToken> {
        // Get `duration` string if option is set
        let mut duration_string = AuthorizationTime::permanent.to_string();
        if let Some(duration) = duration {
            duration_string = duration.to_string();
        }
        // build authorization HashMap
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("response_type".to_owned(), "code".to_owned());
        params.insert("duration".to_owned(), duration_string);
        params.insert("scope".to_owned(), scope.to_owned());
        params.insert("state".to_owned(), self.state_string.clone());
        params.insert(
            "client_id".to_owned(),
            self.client_credentials.client_id.to_owned(),
        );
        params.insert("redirect_uri".to_owned(), self.callback_url.clone());
        let query_string = convert_map_to_string(&params);
        let authorize_url = format!("https://www.reddit.com/api/v1/authorize?{}", query_string);
        match open_browser(&authorize_url) {
            Err(e) => {
                self.oauth_state = OAuthState::ERROR;
                self.error_string = Some(e);
                return None;
            }
            _ => {}
        }
        match get_browser_response(
            120,
            &self.state_string,
            &self.callback_url,
            &self.client_credentials,
        ) {
            Ok(token) => {
                self.oauth_state = OAuthState::AUTHORIZED;
                return Some(token);
            }
            Err(e) => {
                self.error_string = Some(e);
                self.oauth_state = OAuthState::ERROR;
                return None;
            }
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
