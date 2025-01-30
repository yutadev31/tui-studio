use thiserror::Error;

use crate::api::Request;

use super::PluginInfo;

#[derive(Debug, Error)]
pub enum PluginClientError {
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct PluginClient {}

impl PluginClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn initialize(&self, info: PluginInfo) -> Result<(), PluginClientError> {
        let request = serde_json::to_string(&Request {
            command: "initialize".to_string(),
            content: info,
        })?;

        println!("{}", request);
        Ok(())
    }
}
