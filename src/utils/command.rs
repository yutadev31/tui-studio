use std::collections::HashMap;

use crate::action::AppAction;

#[derive(Debug, Clone)]
pub struct CommandManager {
    commands: HashMap<String, Vec<AppAction>>,
}

impl CommandManager {
    pub fn register(&mut self, command: &str, actions: Vec<AppAction>) {
        self.commands.insert(command.to_string(), actions);
    }

    pub fn get_action(&self, command: &str) -> Option<&Vec<AppAction>> {
        self.commands.get(&command.to_string())
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
}
