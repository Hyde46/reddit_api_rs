extern crate reddit_api;

use reddit_api::*;

fn main() {
    println!("Reddit API Testing :)");
    //Reddit OAuth CredentialsManager
    let mut reddit_oauth = reddit_api::oauth2::RedditOAuth::default().build();
    let bearer_token =
        reddit_oauth.authorize_client("identity", Some(oauth2::AuthorizationTime::permanent));
    if let Some(bearer_token) = bearer_token {
        println!("Bearer Token: {:?}", bearer_token);
        let refreshed_token = reddit_oauth.refresh_token(&bearer_token.refresh_token);
        if let Some(refreshed_token) = refreshed_token {
            println!("First Refresh: {:?}", refreshed_token);
            let refreshed_token_again = reddit_oauth.refresh_token(&refreshed_token.refresh_token);
            if let Some(rt) = refreshed_token_again {
                println!("Second Refresh: {:?}", rt);
            }
        }
    }
    //Create Reddit Api object
    //let reddit = reddit_api::client::Reddit::default().client_credentials(client_credentials).build();
    // let post_amount = 100;
    // let posts = reddit.get_frontpage_posts(post_amount);
    //reddit.authorize_reddit_user(None);
}
