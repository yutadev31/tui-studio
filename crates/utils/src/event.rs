use crossterm::event::Event as CrosstermEvent;

pub enum Event {
    Open,                           // Windowを開いた
    Close,                          // Windowを閉じた
    Resize,                         // Windowのサイズが変更された
    Command(String),                // 特定のidのCommandを実行した
    Click(usize, usize),            // 特定の場所をクリック
    CrosstermEvent(CrosstermEvent), // crosstermのイベント
}
