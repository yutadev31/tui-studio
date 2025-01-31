use html::HTMLLanguageSupport;
use tui_studio::api::{PluginClient, PluginClientError, PluginInfo};

fn main() -> Result<(), PluginClientError> {
    let mut client = PluginClient::new();
    client.initialize(PluginInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })?;
    client.set_language_support(Box::new(HTMLLanguageSupport::default()))?;
    client.run()
}
