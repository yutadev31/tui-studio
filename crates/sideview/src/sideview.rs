use utils::{event::Event, window::Window};

pub struct SideView {}

impl Window for SideView {
    fn on_event(&mut self, evt: Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
