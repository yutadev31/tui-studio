use std::{fs::read_dir, path::PathBuf};

use anyhow::Result;
use utils::{event::Event, window::Window};

pub struct FileManager {
    path: PathBuf,
}

impl FileManager {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Window for FileManager {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        Ok(())
    }

    fn draw(&self) -> Result<()> {
        Ok(())
    }
}
