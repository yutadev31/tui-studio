use anyhow::Result;

use crate::event::Event;

pub trait Window {
    fn on_event(&mut self, evt: Event) -> Result<()>;
    fn draw(&self) -> Result<()>;
}
