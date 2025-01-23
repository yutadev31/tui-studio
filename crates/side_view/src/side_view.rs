use anyhow::Result;
use command::CommandManager;
use utils::{
    component::{CommandComponent, Component, DrawableComponent},
    event::Event,
};

pub struct SideView {}

impl SideView {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Component for SideView {
    fn on_event(&mut self, evt: Event) -> anyhow::Result<()> {
        Ok(())
    }
}

impl DrawableComponent for SideView {
    fn draw(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl CommandComponent for SideView {
    fn register_commands(&self, cmd_manager: &mut CommandManager) {}
}
