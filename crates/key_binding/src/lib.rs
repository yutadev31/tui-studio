use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Key {
    Char(char),
    Ctrl(char),
}

pub type KeySequence = Vec<Key>;

pub struct KeyConfig {
    bindings: HashMap<KeySequence, String>,
}

impl KeyConfig {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn register(&mut self, sequence: KeySequence, command: &str) {
        self.bindings.insert(sequence, command.to_string());
    }

    pub fn get_command(&self, sequence: &KeySequence) -> Option<&String> {
        self.bindings.get(sequence)
    }
}
