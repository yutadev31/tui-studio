use anyhow::Result;

use crate::event::Event;

pub trait Component {
    fn on_event(&mut self, evt: Event) -> Result<()>;
}

pub trait DrawableComponent: Component {
    fn draw(&self) -> Result<()>;
}
