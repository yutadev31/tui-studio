use anyhow::Result;

use crate::event::Event;

pub trait Component: Send {
    fn on_event(&mut self, evt: Event) -> Result<Vec<Event>>;
}

pub trait DrawableComponent: Component {
    fn draw(&self) -> Result<()>;
}
