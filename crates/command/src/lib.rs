pub struct CommandManager {
    commands: Vec<String>,
}

impl CommandManager {
    pub fn register(&mut self, id: &str) {
        self.commands.push(id.to_string());
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}
