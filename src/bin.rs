extern crate rsreddit;

use rsreddit::client::Reddit;
use rsreddit::model::sort_time::SortTime;
use rsreddit::oauth2::{AuthorizationTime, RedditApiScope, RedditOAuth};
use rsreddit::util::convert_scope_vec_to_string;

fn main() {
    let mut reddit_oauth = RedditOAuth::default().build();
    // Defines which endpoints the bearer token is allowed to access
    let mut scopes = Vec::new();
    scopes.push(RedditApiScope::identity);
    scopes.push(RedditApiScope::read);
    let scope_string = convert_scope_vec_to_string(&scopes);
    // Authenticate user. Returns bearer token
    let bearer_token =
        reddit_oauth.authorize_client(&scope_string, Some(AuthorizationTime::permanent));
    if let Some(token) = bearer_token {
        let reddit = Reddit::default().bearer_token(token).build();
        let answer = reddit.top(None, SortTime::all, "", "", 0, 20, false, false);
        match answer {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
}
