// use crossterm::event::Event as CrosstermEvent;

use algebra::vec2::{i16::I16Vec2, u16::U16Vec2};

use crate::action::AppAction;

use super::key_binding::Key;

#[derive(Debug, Clone)]
pub enum Event {
    Quit,              // 終了
    Open,              // Windowを開いた
    Close,             // Windowを閉じた
    Resize,            // Windowのサイズが変更された
    Action(AppAction), // 特定のidのCommandを実行した
    Click(U16Vec2),    // 特定の場所をクリック
    Scroll(I16Vec2),
    Input(Key),
    Command(String),
}
