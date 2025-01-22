use anyhow::Result;
use editor::Editor;
use side_view::SideView;
use utils::{event::Event, window::Window};

pub struct App {
    editor: Editor,
    side_view: SideView,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            editor: Editor::new()?,
            side_view: SideView::new()?,
        })
    }
}

impl Window for App {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        self.editor.on_event(evt.clone())?;
        self.side_view.on_event(evt)?;

        Ok(())
    }

    fn draw(&self) -> Result<()> {
        self.editor.draw()?;
        self.side_view.draw()?;

        Ok(())
    }
}
