use std::{collections::HashMap, hash::Hash};

use utils::key::Key;

pub type KeySequence = Vec<Key>;

#[derive(Debug, Default)]
pub struct KeyConfig<Action: Eq + Hash> {
    bindings: HashMap<KeySequence, Action>,
}

impl<Action: Eq + Hash> KeyConfig<Action> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: KeySequence, action: Action) {
        self.bindings.insert(key, action);
    }

    pub fn get_action(&self, key: &KeySequence) -> Option<&Action> {
        self.bindings.get(key)
    }
}
