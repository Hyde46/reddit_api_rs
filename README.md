# RsReddit - A Rust Wrapper for the Reddit Web API

## Description

RsReddit is lightweight wrapper for the Reddit Web API. The goal is to cover **most common and useful endpoints** like browsing subreddits, creating posts, commenting on posts. For a complete reference, see the [official documentation](https://www.reddit.com/dev/api/oauth)

## Features

Implemented endpoints:
* Get sorted posts of subreddit or frontpage
    * Top posts
    * Hot posts
    * Best posts
    * Rising posts
    * New posts
    * Controversial posts
* Comment on thread
* Reply to comments

OAuth2 specific implementations:
* Authorize Client with Reddit UI ( Installed APP type. See [here](https://github.com/reddit-archive/reddit/wiki/oauth2-app-types) )
* Refresh bearer token
* Revoke bearer token

## Usage
Add to your `Cargo.toml`:

``` toml
[dependencies]
rsreddit = "0.1.2"
```

Or get it from [GitHub](https://github.com/Hyde46/reddit_api_rs).

## Getting Started

### Authorization
Currently only Installed Apps are supported.
A script using this library has to register an installed app, or authorize an existing one.
To use an existing App, or set your own app, set the environment variable CLIENT_ID as the app's ID, and CLIENT_SECRET as base64 encoded CLIENT_ID.
You can also set these values in a `.env` file, for example as shown [here](https://github.com/Hyde46/reddit_api_rs/blob/master/.env).

If you want to use your own installed app, create one over [here](https://www.reddit.com/prefs/apps). It is important to choose **installed app**.
See the [official documentation](https://github.com/reddit-archive/reddit/wiki/oauth2)

### Examples
* Reddit API endpoints
    * [Top posts](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/top_posts.rs)
    * [Best posts](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/best_posts.rs)
    * [Comment on post or reply to comment](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/comment_thread.rs)
    * [Traverse comment tree](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/traverse_comment_tree.rs)
* Authorization Examples
    * [Authorize User](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/authorize_user.rs)
    * [Refresh bearer token](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/refresh_token.rs)
    * [Revoke bearer token](https://github.com/Hyde46/reddit_api_rs/blob/master/examples/revoke_token.rs)

## Changelog
See [changelog](https://github.com/Hyde46/reddit_api_rs/blob/master/CHANGELOG.md) for release history

## Contribution

Pull requetss, code reviews, general feedback are welcome!
