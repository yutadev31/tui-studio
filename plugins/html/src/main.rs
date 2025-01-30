use tui_studio::api::{PluginClient, PluginClientError, PluginInfo};

fn main() -> Result<(), PluginClientError> {
    let client = PluginClient::new();
    client.initialize(PluginInfo::default())?;

    Ok(())
}
