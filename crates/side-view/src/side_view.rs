use anyhow::Result;
use utils::{event::Event, window::Window};

pub struct SideView {}

impl SideView {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Window for SideView {
    fn on_event(&mut self, evt: Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
