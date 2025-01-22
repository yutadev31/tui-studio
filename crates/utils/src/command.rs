use anyhow::Result;

pub struct Command {
    callback: Box<dyn Fn() -> Result<()>>,
}

impl Command {
    pub fn new(callback: Box<dyn Fn() -> Result<()>>) -> Self {
        Self { callback }
    }

    pub fn run(&self) -> Result<()> {
        self.callback.as_ref()()
    }
}
