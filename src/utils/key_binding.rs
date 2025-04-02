use std::{collections::HashMap, hash::Hash};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{action::AppAction, editor::core::mode::EditorMode};

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

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum KeyConfigType {
    All,
    Normal,
    Visual,
    NormalAndVisual,
    Insert,
    Command,
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

pub type KeySequence = Vec<Key>;

#[derive(Default)]
pub struct KeyConfig {
    bindings: HashMap<(KeyConfigType, KeySequence), AppAction>,
}

impl KeyConfig {
    pub fn register(
        &mut self,
        config_type: KeyConfigType,
        sequence: KeySequence,
        action: AppAction,
    ) {
        self.bindings.insert((config_type, sequence), action);
    }

    fn get_binding(&self, config_type: KeyConfigType, sequence: KeySequence) -> Option<&AppAction> {
        self.bindings.get(&(config_type, sequence))
    }

    pub fn get_action(&self, mode: EditorMode, sequence: KeySequence) -> Option<&AppAction> {
        self.get_binding(KeyConfigType::All, sequence.clone())
            .or(match mode {
                EditorMode::Normal => self
                    .get_binding(KeyConfigType::Normal, sequence.clone())
                    .or(self.get_binding(KeyConfigType::NormalAndVisual, sequence)),
                EditorMode::Visual { .. } => self
                    .get_binding(KeyConfigType::Visual, sequence.clone())
                    .or(self.get_binding(KeyConfigType::NormalAndVisual, sequence)),
                EditorMode::Command => self.get_binding(KeyConfigType::Command, sequence),
                EditorMode::Insert { .. } => self.get_binding(KeyConfigType::Insert, sequence),
            })
    }
}

