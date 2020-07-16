//Built in libraries
use std::io::Read;

//Third party libraries
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//Own stuff
use super::model::token::OAuthToken;
use super::VERSION;

/// POST Curl request
/// # Arguments
///
/// * `complete_url` - url for curl request
/// * `payload` - payload for post request, if request uses POST
/// * `header` - header data
pub fn post(complete_url: &str, payload: &str, header: &str) -> String {
    let user_agent_header = format!("User-Agent: reddit_api/{}", VERSION);
    let mut easy = Easy::new();

    easy.url(&complete_url).unwrap();
    easy.useragent(&user_agent_header).unwrap();

    // Set Header
    let mut list = List::new();
    list.append(header).unwrap();
    easy.http_headers(list).unwrap();

    // Set post payload
    let mut data_field = payload.as_bytes();
    easy.post(true).unwrap();
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
    return html;
}

/// GET Curl request
/// # Arguments
///
/// * `complete_url` - url for curl request
/// * `header` - header data
pub fn get(complete_url: &str, header: &str) -> String {
    let user_agent_header = format!("User-Agent: reddit_api/{}", VERSION);
    let mut easy = Easy::new();

    easy.url(&complete_url).unwrap();
    easy.useragent(&user_agent_header).unwrap();

    // Set Header
    let mut list = List::new();
    list.append(header).unwrap();
    easy.http_headers(list).unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                html = String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    };
    return html;
}
