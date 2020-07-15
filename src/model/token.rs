use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub scope: String,
    #[serde(default = "empty_string")]
    pub refresh_token: String,
}

fn empty_string() -> String {
    "".to_string()
}
