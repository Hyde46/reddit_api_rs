//! OAuth2 authentication methods for reddit API

//Built in libraries
use std::collections::HashMap;
use std::env;
use std::fmt;

//Third party libraries
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

//Own stuff
use super::callback_server::get_browser_response;
use super::curl_utils::post;
use super::model::token::OAuthToken;
use super::util::convert_map_to_string;
use super::util::generate_random_string;
use super::util::open_browser;

#[allow(non_camel_case_types)]
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
/// See https://www.reddit.com/api/v1/scopes for a list of scopes and their usages
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
    // If OAuthClient is in error state, then displays last error
    pub error_string: Option<String>,
    // Randomly generated string to validate reddit oauth responses
    pub state_string: String,
    // Credentials for reddit_api application
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
    /// Validate RedditOAuth object. After calling, object is ready to use
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
    /// Authorize user by opening default browser and present reddit authorization dialog.
    /// This needs user interaction to receive Bearer Token
    /// # Arguments
    ///
    /// * `scope` - String of concatenated scopes the bearer token should have authorization of
    /// * `duration` - AuthorizationTime::permanent or AuthorizationTime::temporary
    ///
    /// # Returns
    /// `Option<OAuthToken>` If authorization was successfull, OAuthToken is set, otherwise `RedditOAuth.oauth_state=OAuthState::error` with an error message in `RedditOAuth.error_string`
    /// If duration is `permanent`, bearer token will be invalid after one hour. The token has to be refreshed with the `refresh_token`
    pub fn authorize_client(
        &mut self,
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
        // Open Dialog window
        match open_browser(&authorize_url) {
            Err(e) => {
                self.oauth_state = OAuthState::ERROR;
                self.error_string = Some(e);
                return None;
            }
            _ => {}
        }
        // Start local proxy server and wait for reddit response
        match get_browser_response(120, &self.state_string) {
            Ok(code) => {
                self.oauth_state = OAuthState::AUTHORIZED;
                return self.get_initial_access_token(&code);
            }
            Err(e) => {
                self.error_string = Some(e);
                self.oauth_state = OAuthState::ERROR;
                return None;
            }
        }
    }

    /// Refresh bearer token when the previous one expired
    /// # Arguments
    ///
    /// * `to_refresh` - Old bearer token which should be refreshed
    /// # Returns
    /// `Option<OAuthToken>` If authorization was successfull, OAuthToken is set, otherwise `RedditOAuth.oauth_state=OAuthState::error` with an error message in `RedditOAuth.error_string`
    /// If duration is `permanent`, bearer token will be invalid after one hour. The token has to be refreshed with the `refresh_token`
    pub fn refresh_token(&mut self, to_refresh: &OAuthToken) -> Option<OAuthToken> {
        if to_refresh.refresh_token == "" {
            // Token not refreshable
            self.error_string = Some("Token not refreshable `refresh_token` is empty".to_string());
            self.oauth_state = OAuthState::ERROR;
            return None;
        }
        let base_url = "https://www.reddit.com/api/v1/access_token";
        let data_string = format!(
            "grant_type=refresh_token&refresh_token={}",
            to_refresh.refresh_token
        );
        let data_header = format!(
            "Authorization: Basic {}",
            self.client_credentials.client_secret
        );
        let answer = post(base_url, &data_string, &data_header);
        let bearer_token: OAuthToken = serde_json::from_str(&answer).unwrap();
        // Reddit API does not return a value for the refresh token again.
        // Add old `refresh_token` value to newly generated token
        return Some(OAuthToken {
            access_token: bearer_token.access_token,
            token_type: bearer_token.token_type,
            expires_in: bearer_token.expires_in,
            scope: bearer_token.scope,
            refresh_token: to_refresh.refresh_token.to_string(),
        });
    }

    /// Revoke a token by hand.
    /// # Arguments
    ///
    /// * `to_revoke` - Token to revoke its access
    pub fn revoke_token(&mut self, to_revoke: &OAuthToken) -> Result<(), String> {
        let base_url = "https://www.reddit.com/api/v1/revoke_token";
        let data_string = format!(
            "token={}&token_type_hint=access_token",
            to_revoke.access_token
        );
        let data_header = format!(
            "Authorization: Basic {}",
            self.client_credentials.client_secret
        );
        let answer = post(base_url, &data_string, &data_header);
        // Only one reason for non-empty response exists
        if answer != "" {
            return Err(
                "Client credentials sent as HTTP Basic Authorization were invalid".to_string(),
            );
        }
        Ok(())
    }

    /// Request access token
    /// # Arguments
    ///
    /// * `state` - State string returned by reddit authorization process
    ///
    /// # Returns
    /// `Option<OAuthToken>` If authorization was successfull, OAuthToken is set, otherwise `RedditOAuth.oauth_state=OAuthState::error` with an error message in `RedditOAuth.error_string`
    /// If duration is `permanent`, bearer token will be invalid after one hour. The token has to be refreshed with the `refresh_token`
    pub fn get_initial_access_token(&self, state: &str) -> Option<OAuthToken> {
        let data_field_string = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            state, self.callback_url
        );
        let data_header = format!(
            "Authorization: Basic {}",
            self.client_credentials.client_secret
        );
        let base_url = "https://www.reddit.com/api/v1/access_token";
        let answer = post(base_url, &data_field_string, &data_header);
        let bearer_token: OAuthToken = serde_json::from_str(&answer).unwrap();
        return Some(bearer_token);
    }
}

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
