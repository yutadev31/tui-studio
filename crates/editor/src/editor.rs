pub(crate) mod buf;
pub(crate) mod buf_manager;
pub(crate) mod cursor;
pub(crate) mod mode;

use std::io::stdout;

use anyhow::Result;
use buf_manager::EditorBufferManager;
use command::CommandManager;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use key_binding::{Key, KeyConfig};
use mode::EditorMode;
use utils::{component::Component, event::Event, rect::Rect, term::get_term_size};
use utils::{
    component::{CommandComponent, DrawableComponent, KeybindingComponent},
    term::safe_exit,
};

pub struct Editor {
    rect: Rect,
    buffer_manager: EditorBufferManager,
    mode: EditorMode,
}

impl Editor {
    pub fn new(path: Option<String>, rect: Rect) -> Result<Self> {
        Ok(Self {
            rect,
            buffer_manager: EditorBufferManager::new(path)?,
            mode: EditorMode::Normal,
        })
    }
}

impl Component for Editor {
    fn on_event(&mut self, evt: Event) -> Result<()> {
        let (term_w, term_h) = get_term_size()?;

        self.rect.w = term_w;
        self.rect.h = term_h - 1;

        let current = self.buffer_manager.get_current_mut();

        match evt.clone() {
            Event::Command(cmd) => match cmd.as_str() {
                "editor.quit" => safe_exit(),
                "editor.mode.normal" => self.mode = EditorMode::Normal,
                "editor.mode.command" => self.mode = EditorMode::Command,
                "editor.mode.insert" => self.mode = EditorMode::Insert,
                _ => {}
            },
            _ => {}
        }

        if let Some(current) = current {
            current.on_event(evt, &self.mode)?;
        }

        Ok(())
    }
}

impl DrawableComponent for Editor {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        if let Some(buffer) = self.buffer_manager.get_current() {
            let (cursor_x, cursor_y) = buffer.get_cursor_location();
            let (cursor_x, cursor_y): (u16, u16) = (cursor_x as u16, cursor_y as u16);

            let (_scroll_x, scroll_y) = buffer.get_scroll_location();
            let lines = buffer.get_lines();

            for (index, line) in lines
                .iter()
                .skip(scroll_y)
                .take(self.rect.h.into())
                .enumerate()
            {
                let y: u16 = index.try_into()?;
                execute!(stdout(), MoveTo(self.rect.x, self.rect.y + y), Print(line))?;
            }

            let scroll_y: u16 = scroll_y.try_into()?;
            execute!(
                stdout(),
                MoveTo(cursor_x + self.rect.x, cursor_y - scroll_y + self.rect.y)
            )?;
        }

        Ok(())
    }
}

impl CommandComponent for Editor {
    fn register_commands(&self, cmd_manager: &mut CommandManager) {
        cmd_manager.register("editor.quit");

        // Editor Mode
        cmd_manager.register("editor.mode.normal");
        cmd_manager.register("editor.mode.command");
        cmd_manager.register("editor.mode.insert");

        // Editor Cursor
        cmd_manager.register("editor.cursor.left");
        cmd_manager.register("editor.cursor.down");
        cmd_manager.register("editor.cursor.up");
        cmd_manager.register("editor.cursor.right");
        cmd_manager.register("editor.cursor.line_start");
        cmd_manager.register("editor.cursor.line_end");
        cmd_manager.register("editor.cursor.top");
        cmd_manager.register("editor.cursor.end");
    }
}

impl KeybindingComponent for Editor {
    fn register_keybindings(&self, key_config: &mut KeyConfig) {
        key_config.register(vec![Key::Char('q')], "editor.quit");
        key_config.register(vec![Key::Ctrl('c')], "editor.mode.normal");
        key_config.register(vec![Key::Char(':')], "editor.mode.command");
        key_config.register(vec![Key::Char('i')], "editor.mode.insert");
        key_config.register(vec![Key::Char('h')], "editor.cursor.left");
        key_config.register(vec![Key::Char('j')], "editor.cursor.down");
        key_config.register(vec![Key::Char('k')], "editor.cursor.up");
        key_config.register(vec![Key::Char('l')], "editor.cursor.right");
        key_config.register(vec![Key::Char('0')], "editor.cursor.line_start");
        key_config.register(vec![Key::Char('$')], "editor.cursor.line_end");
        key_config.register(vec![Key::Char('g'), Key::Char('g')], "editor.cursor.top");
        key_config.register(vec![Key::Char('G')], "editor.cursor.end");
    }
}
