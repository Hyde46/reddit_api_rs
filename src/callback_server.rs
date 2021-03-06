//Built in libraries
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//Thrid party libraries
use dotenv::dotenv;
use tiny_http::Server;
use tiny_http::ServerConfig;

use super::util::chomp_http_prefix;

struct ResponseBox {
    has_error: bool,
    error_msg: String,
    code: String,
    state: String,
}

pub fn get_browser_response(auth_time: usize, client_state: &str) -> Result<String, String> {
    let mut error_msg = "";
    let (tx_authentication, rx) = mpsc::channel();
    let tx_countdown = mpsc::Sender::clone(&tx_authentication);
    thread::spawn(move || {
        dotenv().ok();
        let mut callback_url = env::var("REDIRECT_URI").unwrap_or_default();
        // Reddit expects the callback url to start with "http://" or "https://", However, tinyhttp does not
        // not support url to start with either prefixes, but rather Server::http or Server::https gets called.
        // thus, the prefix has to be removed if it exists
        callback_url = chomp_http_prefix(&callback_url);
        let server = Server::http(callback_url).unwrap();
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
            error_msg: "Reached timeout. User did not authorize usage of reddit_api in time"
                .to_string(),
            code: "".to_string(),
            state: "".to_string(),
        };
        trace!("Timeout during authentication");
        tx_countdown.send(auth_box).unwrap();
    });
    let response_box = rx.recv().unwrap();

    if response_box.has_error {
        return Err(response_box.error_msg);
    } else {
        if client_state != response_box.state {
            return Err(
                "State string of response is not the same. Cannot trust the bearer token."
                    .to_owned(),
            );
        } else {
            return Ok(response_box.code);
        }
    }
}
