pub(crate) mod buf;

use std::io::stdout;

use anyhow::Result;
use buf::buf_manager::EditorBufferManager;
use command::CommandManager;
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use key_binding::{component::KeybindingComponent, Key, KeyConfig};
use utils::{
    component::Component, event::Event, mode::EditorMode, rect::Rect, term::get_term_size,
};
use utils::{
    component::{CommandComponent, DrawableComponent},
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

    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
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
                _ => {}
            },
            _ => {}
        }

        if let Some(current) = current {
            match evt.clone() {
                Event::Command(cmd) => match cmd.as_str() {
                    "editor.mode.normal" => {
                        if let EditorMode::Insert { append } = self.mode {
                            if append {
                                current.cursor_move_by(-1, 0, &self.mode)?;
                            }
                        }
                        self.mode = EditorMode::Normal;
                        current.cursor_sync(&self.mode);
                    }
                    _ => {}
                },
                _ => {}
            }

            match self.mode {
                EditorMode::Normal => match evt.clone() {
                    Event::Command(cmd) => match cmd.as_str() {
                        "editor.mode.command" => {
                            self.mode = EditorMode::Command;
                            current.cursor_sync(&self.mode);
                        }
                        "editor.mode.insert" => {
                            self.mode = EditorMode::Insert { append: false };
                            current.cursor_sync(&self.mode);
                        }
                        "editor.mode.append" => {
                            self.mode = EditorMode::Insert { append: true };
                            current.cursor_move_by(1, 0, &self.mode)?;
                            current.cursor_sync(&self.mode);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }

            current.on_event(evt, &self.mode)?;
        }

        Ok(())
    }
}

impl DrawableComponent for Editor {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        if let Some(buffer) = self.buffer_manager.get_current() {
            let (_scroll_x, scroll_y) = buffer.get_scroll_location();
            let lines = buffer.get_code_buf().get_lines();

            for (index, line) in lines
                .iter()
                .skip(scroll_y)
                .take(self.rect.h.into())
                .enumerate()
            {
                let y: u16 = index.try_into()?;
                execute!(stdout(), MoveTo(self.rect.x, self.rect.y + y), Print(line))?;
            }

            execute!(stdout(), Print(self.mode.to_string()))?;

            let (cursor_x, cursor_y) = buffer.get_cursor_draw_location(&self.mode);
            let (cursor_x, cursor_y): (u16, u16) = (cursor_x as u16, cursor_y as u16);

            let scroll_y: u16 = scroll_y.try_into()?;
            execute!(
                stdout(),
                MoveTo(cursor_x + self.rect.x, cursor_y - scroll_y + self.rect.y)
            )?;

            match self.mode {
                EditorMode::Normal => {
                    execute!(stdout(), SetCursorStyle::SteadyBlock)?;
                }
                EditorMode::Insert { append: _ } => {
                    execute!(stdout(), SetCursorStyle::SteadyBar)?;
                }
                EditorMode::Command => {
                    execute!(stdout(), SetCursorStyle::SteadyBar)?;
                }
            }
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
        // key_config.register(EditorMode::Normal, vec![Key::Char('q')], "editor.quit");
        key_config.register(
            EditorMode::Insert { append: false },
            vec![Key::Ctrl('c')],
            "editor.mode.normal",
        );
        key_config.register(
            EditorMode::Insert { append: true },
            vec![Key::Ctrl('c')],
            "editor.mode.normal",
        );
        key_config.register(
            EditorMode::Command,
            vec![Key::Ctrl('c')],
            "editor.mode.normal",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char(':')],
            "editor.mode.command",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('i')],
            "editor.mode.insert",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('a')],
            "editor.mode.append",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('h')],
            "editor.cursor.left",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('j')],
            "editor.cursor.down",
        );
        key_config.register(EditorMode::Normal, vec![Key::Char('k')], "editor.cursor.up");
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('l')],
            "editor.cursor.right",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('0')],
            "editor.cursor.line_start",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('$')],
            "editor.cursor.line_end",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('g'), Key::Char('g')],
            "editor.cursor.top",
        );
        key_config.register(
            EditorMode::Normal,
            vec![Key::Char('G')],
            "editor.cursor.end",
        );
    }
}
