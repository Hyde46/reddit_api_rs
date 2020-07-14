//Built in libraries
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//Thrid party libraries
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use tiny_http::{Response, Server};

//Own libraries
use super::oauth2::OAuthToken;
use super::oauth2::RedditClientCredentials;

struct ResponseBox {
    has_error: bool,
    error_msg: String,
    code: String,
    state: String,
}

pub fn get_browser_response(
    auth_time: usize,
    client_state: &str,
    callback_url: &str,
    client_credentials: &RedditClientCredentials,
) -> Result<OAuthToken, String> {
    let mut error_msg = "";
    let (tx_authentication, rx) = mpsc::channel();
    let tx_countdown = mpsc::Sender::clone(&tx_authentication);
    thread::spawn(move || {
        let server = Server::http("127.0.0.1:8000").unwrap();
        for request in server.incoming_requests() {
            let request_url = request.url().to_string().clone();
            let parameter_string: Vec<&str> = request_url.split("/?").collect();
            if parameter_string.len() <= 1 {
                continue;
            };
            let parameters: Vec<&str> = parameter_string[1].split('&').collect();
            // Expect state and code parameters
            if parameters.len() != 2 {
                error_msg = "Unexpected response from reddit";
                let auth_box = ResponseBox {
                    has_error: true,
                    error_msg: "Unexpected response from reddit".to_string(),
                    code: "".to_string(),
                    state: "".to_string(),
                };
                tx_authentication.send(auth_box).unwrap();
            } else {
                let state: Vec<&str> = parameters[0].split('=').collect();
                let code: Vec<&str> = parameters[1].split('=').collect();
                let auth_box = ResponseBox {
                    has_error: false,
                    error_msg: "".to_string(),
                    code: code[1].to_string(),
                    state: state[1].to_string(),
                };
                tx_authentication.send(auth_box).unwrap();
            }
        }
        drop(server);
    });
    thread::spawn(move || {
        for _ in 0..auth_time {
            thread::sleep(Duration::from_secs(1));
        }
        let auth_box = ResponseBox {
            has_error: true,
            error_msg: "Reached timeout. User did not authorize usage of reddit_api in time".to_string(),
            code: "".to_string(),
            state: "".to_string(),
        };
        println!("Timeout during authentication");
        tx_countdown.send(auth_box).unwrap();
    });
    let response_box = rx.recv().unwrap();

    // Getting Access token now

    let data_field_string = format!(
        "grant_type=authorization_code&code={}&redirect_uri={}",
        response_box.code, callback_url
    );
    let mut data_field = data_field_string.as_bytes();
    let mut list = List::new();
    let data_header = format!("Authorization: Basic {}",client_credentials.client_secret);
    list.append(&data_header).unwrap();

    let user_agent_header = "User-Agent: reddit_api/0.1 by rust_reddit_api";
    let mut easy = Easy::new();
    easy.url("https://www.reddit.com/api/v1/access_token")
        .unwrap();
    easy.http_headers(list).unwrap();
    easy.post(true).unwrap();
    easy.useragent(user_agent_header).unwrap();
    easy.post_field_size(data_field.len() as u64).unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(data_field.read(buf).unwrap_or(0)))
            .unwrap();
        transfer
            .write_function(|data| {
                html = String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    };
    let bearer_token: OAuthToken = serde_json::from_str(&html).unwrap();

    if response_box.has_error {
        return Err(response_box.error_msg);
    } else {
        if client_state != response_box.state {
            return Err(
                "State string of response is not the same. Cannot trust the bearer token."
                    .to_owned(),
            );
        } else {
            return Ok(bearer_token);
        }
    }
}
