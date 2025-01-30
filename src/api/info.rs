use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}

impl Default for PluginInfo {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
