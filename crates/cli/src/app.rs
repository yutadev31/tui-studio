use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use command::CommandManager;
use crossterm::event::Event as CrosstermEvent;
use editor::Editor;
use key_binding::{Key, KeyConfig};
use side_view::SideView;
use utils::{
    component::{CommandComponent, Component, DrawableComponent},
    event::Event,
    rect::Rect,
    term::get_term_size,
};

use key_binding::component::KeybindingComponent;

pub struct App {
    editor: Editor,
    side_view: SideView,

    cmd_manager: CommandManager,
    key_config: KeyConfig,

    first_key_time: Option<DateTime<Utc>>,
    key_buf: Vec<Key>,
}

impl App {
    pub fn new(path: Option<String>) -> Result<Self> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            editor: Editor::new(path, Rect::new(0, 0, term_w, term_h))?,
            side_view: SideView::new()?,
            cmd_manager: CommandManager::default(),
            key_config: KeyConfig::default(),
            key_buf: Vec::new(),
            first_key_time: None,
        })
    }

    pub fn init(&mut self) {
        // Editor
        self.editor.register_commands(&mut self.cmd_manager);
        self.editor.register_keybindings(&mut self.key_config);

        // Side View
        self.side_view.register_commands(&mut self.cmd_manager);
    }
}

impl Component for App {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        if let Event::CrosstermEvent(evt) = evt.clone() {
            if let CrosstermEvent::Key(evt) = evt {
                if self.key_buf.len() == 0 {
                    self.first_key_time = Some(Utc::now())
                } else if let Some(first_key_time) = self.first_key_time {
                    let now = Utc::now();
                    let elapsed = now - first_key_time;

                    if elapsed >= Duration::milliseconds(500) {
                        self.key_buf = Vec::new();
                    }
                }

                self.key_buf.push(Key::from(evt));

                match self
                    .key_config
                    .get_command(self.editor.get_mode(), self.key_buf.clone())
                {
                    None => {}
                    Some(command) => {
                        self.key_buf = Vec::new();
                        self.on_event(Event::Command(command.clone()))?;
                        return Ok(());
                    }
                };
            }
        }

        self.editor.on_event(evt.clone())?;
        self.side_view.on_event(evt)?;

        Ok(())
    }
}

impl DrawableComponent for App {
    fn draw(&self) -> Result<()> {
        self.editor.draw()?;
        self.side_view.draw()?;

        Ok(())
    }
}
