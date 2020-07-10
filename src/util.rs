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

/// Generates a random string of `length` from Alphanumeric values
pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}
