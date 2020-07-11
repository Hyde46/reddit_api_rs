use std::collections::HashMap;
use std::fmt;
use std::string::String;

use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::util::convert_map_to_string;

use super::oauth2::RedditApiScope;
use super::oauth2::RedditClientCredentials;
use super::oauth2::RedditOAuthClient;

#[derive(PartialEq, Debug)]
/// Determines during client authorization whether the token is permanent or temporary
pub enum AuthorizationTimeOption {
    permanent,
    temporary,
}
impl fmt::Display for AuthorizationTimeOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reddit API client.
/// Allows to communicate with reddit REST and oauth2 endpoints
/// See: https://www.reddit.com/dev/api/oauth/
/// Endpoints are partially implemented.
/// See github repository for current implementations and roadmap
pub struct Reddit {
    pub oauth_prefix: String,
    pub basic_prefix: String,
    pub client_credentials: RedditClientCredentials,
    pub oauth_client: RedditOAuthClient,
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
            oauth_prefix: "https://www.reddit.com/api/v1/".to_owned(),
            basic_prefix: "https://www.reddit.com/".to_owned(),
            client_credentials: RedditClientCredentials::default(),
            oauth_client: RedditOAuthClient::default(),
            is_built: false,
        }
    }
    /// Sets Reddit OAuth Client if custom ones are wished
    ///
    /// # Example
    /// ```
    /// use reddit_api::client::Reddit;
    /// use reddit_api::oauth2::RedditOAuthClient;
    /// let oauth_client = RedditOAuthClient::default().state_string("123abc");
    /// let reddit = Reddit::default()
    ///                      .oauth_client(&oauth_client);
    /// ```
    pub fn oauth_client(mut self, oauth_client: &RedditOAuthClient) -> Reddit {
        self.oauth_client = oauth_client.clone();
        self
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
    ///                      .oauth_prefix("https://www.alternate_reddit.com/api/v1/");
    /// ```
    pub fn oauth_prefix(mut self, prefix: &str) -> Reddit {
        self.oauth_prefix = prefix.to_owned();
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
    /// Validates Reddit object in a basic manner.
    /// After calling, object is ready to use
    pub fn build(mut self) -> Reddit {
        if self.oauth_prefix == "" || self.basic_prefix == "" {
            panic!("No prefixes provided. Cannot communicate with reddit API endpoint!");
        }
        self.is_built = true;
        self
    }

    /// Prepares GET request
    /// # Arguments
    ///
    /// * `url` - Consists of base url to api endpoint
    /// * `param` - HashMap of parameter name and values
    pub fn get(&self, url: &str, params: &mut HashMap<String, String>) -> Result<String, String> {
        if !self.is_built {
            return Err("Object not built. Run `build()` before calling this method.".to_string());
        }
        if !params.is_empty() {
            /* let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);*/
            // curl here
            return Err("Not implemented".to_string());
        } else {
            //curl here without parameters
            return Err("Not implemented".to_string());
        }
    }

    pub fn post(&self, url: &str, payload: &str) {}
    pub fn put(&self, url: &str, payload: &str) {}
    pub fn delete(&self, url: &str, payload: &str) {}

    fn curl_reddit(&self, complete_url: &str, payload: Option<&str>) -> Result<String, String> {
        if !self.is_built {
            return Err("Object not built. Run `build()` before calling this method.".to_string());
        }
        let user_agent_header = "User-Agent: RVP/0.1 by Gitrog_Frog";
        let mut easy = Easy::new();
        easy.url(&complete_url).unwrap();
        easy.useragent(user_agent_header).unwrap();

        if let Some(p) = payload {
            let mut list = List::new();
            list.append(p).unwrap();
            easy.http_headers(list).unwrap()
        }

        let mut return_data: Vec<String> = Vec::new();
        let mut html: String = String::new();
        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    html = String::from_utf8(Vec::from(data)).unwrap();
                    return_data.push(html.clone());
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        };
        Ok(return_data.join(""))
    }

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
        return Ok("All good".to_string());
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
            reddit.oauth_prefix,
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
        Reddit::default().oauth_prefix("").build();
    }
    #[test]
    #[should_panic(expected = "No prefixes provided. Cannot communicate with reddit API endpoint!")]
    fn test_use_reddit_without_basic_prefix() {
        Reddit::default().basic_prefix("").build();
    }

    #[test]
    fn test_get_but_not_built() {
        let reddit = Reddit::default();
        let result = reddit.get("", &mut HashMap::new());
        assert_eq!(
            result,
            Err("Object not built. Run `build()` before calling this method.".to_string())
        );
    }
}
