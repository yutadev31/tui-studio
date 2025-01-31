use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}
