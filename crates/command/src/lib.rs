pub struct CommandManager {
    commands: Vec<String>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn register(&mut self, id: &str) {
        self.commands.push(id.to_string());
    }
}
