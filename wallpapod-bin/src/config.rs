use serde::{Deserialize, Serialize};

pub const CFG_FILE: &str = "config";

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// The key used for authentication with NASA's APOD API
    pub api_key: String,
}
