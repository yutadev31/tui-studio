pub mod component;

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use utils::mode::EditorMode;

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
            _ => panic!("Invalid key event"),
        }
    }
}

pub type KeySequence = Vec<Key>;

pub struct KeyConfig {
    bindings: HashMap<(EditorMode, KeySequence), String>,
}

impl KeyConfig {
    pub fn register(&mut self, mode: EditorMode, sequence: KeySequence, command: &str) {
        self.bindings.insert((mode, sequence), command.to_string());
    }

    pub fn get_command(&self, mode: EditorMode, sequence: KeySequence) -> Option<&String> {
        self.bindings.get(&(mode, sequence))
    }
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}
