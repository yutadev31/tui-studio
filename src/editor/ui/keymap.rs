use std::collections::HashMap;

use crate::{editor::core::mode::EditorMode, utils::key_binding::Key};

pub struct Keymap {
    keymap: HashMap<(EditorMode, Key), String>,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            keymap: HashMap::new(),
        }
    }

    pub fn nmap(&mut self, key: Key, action: &str) {
        self.keymap
            .insert((EditorMode::Normal, key), action.to_string());
    }

    pub fn set_default_keymap(&mut self) {
        self.nmap(Key::Char('h'), "cursor.left");
        self.nmap(Key::Char('j'), "cursor.down");
        self.nmap(Key::Char('k'), "cursor.up");
        self.nmap(Key::Char('l'), "cursor.right");
    }
}
