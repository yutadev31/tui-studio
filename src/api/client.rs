use std::io::stdin;

use thiserror::Error;

use crate::api::Request;

use super::{language_support::LanguageSupport, PluginInfo};

#[derive(Debug, Error)]
pub enum PluginClientError {
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct PluginClient {
    lang_supports: Vec<Box<dyn LanguageSupport>>,
}

impl PluginClient {
    pub fn new() -> Self {
        Self {
            lang_supports: Vec::new(),
        }
    }

    pub fn initialize(&self, info: PluginInfo) -> Result<(), PluginClientError> {
        let request = serde_json::to_string(&Request {
            command: "initialize".to_string(),
            content: info,
        })?;

        println!("{}", request);
        Ok(())
    }

    fn on_event(&self, _buf: String) {}

    pub fn run(&self) -> Result<(), PluginClientError> {
        let stdin = stdin();
        loop {
            let mut buf = String::new();
            match stdin.read_line(&mut buf) {
                Ok(0) => continue,
                Ok(_) => {
                    self.on_event(buf);
                }
                Err(_) => {}
            }
        }
    }

    pub fn register_language_support(
        &mut self,
        lang_support: Box<dyn LanguageSupport>,
    ) -> Result<(), PluginClientError> {
        self.lang_supports.push(lang_support);
        Ok(())
    }
}
