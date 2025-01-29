pub mod component;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommandManager {
    commands: HashMap<String, Vec<String>>,
}

impl CommandManager {
    pub fn register(&mut self, alias: &str, command: Vec<&str>) {
        self.commands.insert(
            alias.to_string(),
            command.iter().map(|cmd| cmd.to_string()).collect(),
        );
    }

    pub fn get_command(&self, alias: &str) -> Option<&Vec<String>> {
        self.commands.get(&alias.to_string())
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
}
