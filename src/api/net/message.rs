use serde::{Deserialize, Serialize};

use crate::api::{language_support::highlight::HighlightToken, PluginInfo};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    Initialize(PluginInfo),
    SetLanguageSupport(&'static str),
    HighlightRequest,
    HighlightResponse(Vec<HighlightToken>),
}
