use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Backspace,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Esc,
    InvalidKey,
}

fn map_key_with_modifiers(modifiers: KeyModifiers, c: char) -> Key {
    if modifiers.contains(KeyModifiers::CONTROL) {
        Key::Ctrl(c)
    } else {
        Key::Char(c)
    }
}

impl From<KeyEvent> for Key {
    fn from(evt: KeyEvent) -> Self {
        match evt.code {
            KeyCode::Char(c) => map_key_with_modifiers(evt.modifiers, c),
            KeyCode::Enter => map_key_with_modifiers(evt.modifiers, '\n'),
            KeyCode::Tab => map_key_with_modifiers(evt.modifiers, '\t'),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Delete => Key::Delete,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Esc => Key::Esc,
            _ => Key::InvalidKey,
        }
    }
}
