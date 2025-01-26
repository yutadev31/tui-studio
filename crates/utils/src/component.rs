use anyhow::Result;
use command::CommandManager;

use crate::event::Event;

pub trait Component {
    fn on_event(&mut self, evt: Event) -> Result<()>;
}

pub trait DrawableComponent: Component {
    fn draw(&self) -> Result<()>;
}

pub trait CommandComponent: Component {
    fn register_commands(&self, cmd_manager: &mut CommandManager);
}
