use std::collections::HashMap;

use crate::action::AppAction;

#[derive(Debug, Clone)]
pub struct CommandManager {
    commands: HashMap<&'static str, fn(args: Vec<&str>) -> Vec<AppAction>>,
}

impl CommandManager {
    pub fn register(
        &mut self,
        command: &'static str,
        callback: fn(args: Vec<&str>) -> Vec<AppAction>,
    ) {
        self.commands.insert(command, callback);
    }

    pub fn get_action(&self, command: &str) -> Option<Vec<AppAction>> {
        let mut args = command.split_whitespace();
        let command = args.next()?;
        let args: Vec<&str> = args.collect();
        let cmd = self.commands.get(command)?;

        Some(cmd(args))
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
}
