use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}
