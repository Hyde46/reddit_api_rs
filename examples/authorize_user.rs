extern crate rsreddit;

use rsreddit::oauth2::{AuthorizationTime, RedditApiScope, RedditOAuth};
use rsreddit::util::convert_scope_vec_to_string;

fn main() {
    let mut reddit_oauth = RedditOAuth::default().build();
    // Defines which endpoints the bearer token is allowed to access
    let mut scopes = Vec::new();
    scopes.push(RedditApiScope::identity);
    scopes.push(RedditApiScope::subscribe);
    let scope_string = convert_scope_vec_to_string(&scopes);
    // Authenticate user. Returns bearer token
    let bearer_token =
        reddit_oauth.authorize_client(&scope_string, Some(AuthorizationTime::permanent));
    println!("Authenticated!");
}
