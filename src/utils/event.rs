// use crossterm::event::Event as CrosstermEvent;

use crate::action::AppAction;

use super::{
    key_binding::Key,
    vec2::{IVec2, UVec2},
};

#[derive(Debug, Clone)]
pub enum Event {
    Quit,              // 終了
    Open,              // Windowを開いた
    Close,             // Windowを閉じた
    Resize,            // Windowのサイズが変更された
    Action(AppAction), // 特定のidのCommandを実行した
    Click(UVec2),      // 特定の場所をクリック
    Scroll(IVec2),
    Input(Key),
    Command(String),
}
