use std::path::PathBuf;

use anyhow::Result;
use utils::{
    component::{Component, DrawableComponent},
    event::Event,
};

pub struct FileManager {
    path: PathBuf,
}

impl FileManager {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Component for FileManager {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        Ok(())
    }
}

impl DrawableComponent for FileManager {
    fn draw(&self) -> Result<()> {
        Ok(())
    }
}
