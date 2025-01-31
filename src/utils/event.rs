// use crossterm::event::Event as CrosstermEvent;

use super::key_binding::Key;

#[derive(Debug, Clone)]
pub enum Event {
    Quit,                // 終了
    Open,                // Windowを開いた
    Close,               // Windowを閉じた
    Resize,              // Windowのサイズが変更された
    Command(String),     // 特定のidのCommandを実行した
    Click(usize, usize), // 特定の場所をクリック
    Input(Key),
    RunCommand(String),
}
