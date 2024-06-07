use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub aliases: HashMap<String, String>,
    pub paths: Vec<String>,
    pub env_vars: HashMap<String, String>,
}

impl Config {
    pub fn from_ron(raw: &str) -> Config {
        let config: Config = ron::from_str(raw).unwrap();
        config
    }
}
