extern crate rsreddit;

use rsreddit::client::Reddit;
use rsreddit::model::sort_time::SortTime;
use rsreddit::oauth2::{AuthorizationTime, RedditApiScope, RedditOAuth};
use rsreddit::util::convert_scope_vec_to_string;

fn main() {
    // Show hot posts for authorized user
    // Works the same for:
    // * reddit.hot()
    // * reddit.controversial()
    // * reddit.new()
    // * reddit.rising()
    // * reddit.best()

    // OAuth2 Authorization stuff
    let mut reddit_oauth = RedditOAuth::default().build();
    let mut scopes = Vec::new();
    scopes.push(RedditApiScope::read);
    let scope_string = convert_scope_vec_to_string(&scopes);
    // Authenticate user. Returns bearer token
    let bearer_token =
        reddit_oauth.authorize_client(&scope_string, Some(AuthorizationTime::permanent));
    if let Some(token) = bearer_token {
        // Reddit client object
        let reddit = Reddit::default().bearer_token(token).build();
        // Query hot posts of /r/rust with a limit of 20 posts
        let answer = reddit
            .hot(Some("/r/rust"), "", "", 0, 20, false, false)
            .unwrap();
        // Get "after" tag from Listing to browse the following posts
        let after = answer.data.after.unwrap();
        // Query hot posts after previous ones
        let next_answer = reddit.hot(Some("/r/rust"), &after, "", 0, 20, false, false);
        // Do stuff with Listing
        match next_answer {
            Ok(a) => println!("{:?}", a),
            Err(e) => println!("{}", e),
        }
    }
}
