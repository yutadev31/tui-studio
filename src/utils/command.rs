use std::collections::HashMap;

use crate::action::AppAction;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct CommandManager {
    commands: HashMap<String, Vec<AppAction>>,
}

impl CommandManager {
    pub fn register(&mut self, alias: &str, actions: Vec<AppAction>) {
        self.commands.insert(alias.to_string(), actions);
    }

    pub fn get_command(&self, alias: &str) -> Option<&Vec<AppAction>> {
        self.commands.get(&alias.to_string())
    }
}

