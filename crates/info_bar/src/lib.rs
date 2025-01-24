use anyhow::Result;
use utils::{
    component::{Component, DrawableComponent},
    event::Event,
    rect::Rect,
};

pub struct InfoBar {
    rect: Rect,
}

impl InfoBar {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

impl Component for InfoBar {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        Ok(())
    }
}

impl DrawableComponent for InfoBar {
    fn draw(&self) -> Result<()> {
        Ok(())
    }
}
