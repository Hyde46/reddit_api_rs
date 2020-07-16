use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Preview {
    pub enabled: bool,
    pub images: Vec<String>, //TODO: create image model
}
