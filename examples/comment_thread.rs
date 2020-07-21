extern crate rsreddit;

use rsreddit::client::Reddit;
use rsreddit::model::sort_time::SortTime;
use rsreddit::oauth2::{AuthorizationTime, RedditApiScope, RedditOAuth};
use rsreddit::util::convert_scope_vec_to_string;

fn main() {
    // Comment on thread, or reply to comment


    // OAuth2 Authorization stuff
    let mut reddit_oauth = RedditOAuth::default().build();
    let mut scopes = Vec::new();
    scopes.push(RedditApiScope::submit);
    let scope_string = convert_scope_vec_to_string(&scopes);
    // Authenticate user. Returns bearer token
    let bearer_token =
        reddit_oauth.authorize_client(&scope_string, Some(AuthorizationTime::permanent));
    if let Some(token) = bearer_token {
        // Reddit client object
        let reddit = Reddit::default().bearer_token(token).build();
        // Post comment to toplevel thread
        reddit.comment("<thing_id>", "Your comment here :)");
    }
}
