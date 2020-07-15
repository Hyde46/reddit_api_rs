use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::string::ToString;

/// convert map to `query_string`, for example:
/// convert
/// `{"redirect_uri":"my_uri",
///  "state":"my-state"
///  "scope":"test-scope"}`
/// to
/// `redirect_uri=my_uri&state=my-state&scope=test-scope`
/// Since hashmap is not sorted, so the order of key-value-pairs
/// may differ from times
pub fn convert_map_to_string<
    K: Debug + Eq + Hash + ToString,
    V: Debug + ToString,
    S: ::std::hash::BuildHasher,
>(
    map: &HashMap<K, V, S>,
) -> String {
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        string.push_str(&key.to_string());
        string.push_str("=");
        string.push_str(&value.to_string());
        string.push_str("&");
    }
    string
}

/// Converts vector of objects with ToString Trait to one string with one space as spacer
pub fn convert_scope_vec_to_string<S: Debug + ToString>(vec: &Vec<S>) -> String {
    if vec.len() == 0 {
        return "".to_string();
    }
    vec.iter()
        .map(|x| x.to_string() + " ")
        .collect::<String>()
        .trim()
        .to_string()
}

/// Generates a random string of `length` from Alphanumeric values
pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

pub fn open_browser(url: &str) -> Result<(), String> {
    if !webbrowser::open(url).is_ok() {
        return Err("Could not open browser. Is a default browser set?".to_owned());
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::super::oauth2::RedditApiScope;
    use super::*;

    #[test]
    fn test_convert_string_vec_to_string() {
        let v = vec!["scope1", "scope2", "scope3"];
        let concat = convert_scope_vec_to_string(&v);
        assert_eq!("scope1 scope2 scope3", concat);
    }

    #[test]
    fn test_convert_scope_vec_to_string() {
        let v = vec![RedditApiScope::identity, RedditApiScope::modconfig];
        let concat = convert_scope_vec_to_string(&v);
        assert_eq!("identity modconfig", concat);
    }
}
