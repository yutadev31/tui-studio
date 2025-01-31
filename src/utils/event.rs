// use crossterm::event::Event as CrosstermEvent;

use crate::action::AppAction;

use super::key_binding::Key;

#[derive(Debug, Clone)]
pub enum Event {
    Quit,                // 終了
    Open,                // Windowを開いた
    Close,               // Windowを閉じた
    Resize,              // Windowのサイズが変更された
    Action(AppAction),   // 特定のidのCommandを実行した
    Click(usize, usize), // 特定の場所をクリック
    Input(Key),
    Command(String),
}
