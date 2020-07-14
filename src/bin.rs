extern crate reddit_api;

use reddit_api::*;

fn main() {
    println!("Reddit API Testing :)");
    //Reddit OAuth CredentialsManager
    let mut reddit_oauth = reddit_api::oauth2::RedditOAuth::default().build();
    if let Some(bearer_token) =
        reddit_oauth.authorize_client("identity", Some(oauth2::AuthorizationTime::permanent))
    {
        println!("{:?}", bearer_token);
    }
    //Create Reddit Api object
    //let reddit = reddit_api::client::Reddit::default().client_credentials(client_credentials).build();
    // let post_amount = 100;
    // let posts = reddit.get_frontpage_posts(post_amount);
    //reddit.authorize_reddit_user(None);
}
