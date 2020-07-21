# RsReddit - A Rust client for the Reddit Web API

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
* Authorize Client with Reddit UI ( Get OAuth bearer token )
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

### Examples

## Changelog
See [changelog](https://github.com/Hyde46/reddit_api_rs/blob/master/CHANGELOG.md) for release history

## Contribution

Pull requetss, code reviews, general feedback are welcome!