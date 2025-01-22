pub(crate) mod buf;
pub(crate) mod buf_manager;
pub(crate) mod config;
pub(crate) mod cursor;
pub(crate) mod input_manager;
pub(crate) mod mode;

use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use buf_manager::EditorBufferManager;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use mode::EditorMode;
use utils::{
    rect::Rect,
    term::{get_term_size, safe_exit},
};

pub struct Editor {
    rect: Rect,
    buffer_manager: EditorBufferManager,
    mode: EditorMode,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            rect: Rect::new(0, 0, term_w, term_h),
            buffer_manager: EditorBufferManager::new(),
            mode: EditorMode::Normal,
        })
    }

    pub fn on_event(&mut self, evt: Event) -> Result<()> {
        let (term_w, term_h) = get_term_size()?;
        self.rect.w = term_w;
        self.rect.h = term_h;

        match evt {
            Event::Key(evt) => match evt.code {
                KeyCode::Char('q') => safe_exit(),
                KeyCode::Esc => {
                    if !self.mode.eq(&EditorMode::Normal) {
                        self.mode = EditorMode::Normal;
                        return Ok(());
                    }
                }
                KeyCode::Char('i') => {
                    if !self.mode.eq(&EditorMode::Insert) {
                        self.mode = EditorMode::Insert;
                        return Ok(());
                    }
                }
                KeyCode::Char(':') => {
                    if !self.mode.eq(&EditorMode::Command) {
                        self.mode = EditorMode::Command;
                        return Ok(());
                    }
                }
                _ => {}
            },
            _ => {}
        }

        // if let Some(open_buffer) = self.open_buffer {
        //     self.buffers[open_buffer].on_event(evt, &self.mode)?;
        // }

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        if let Some(buffer) = self.buffer_manager.get_current() {
            let (cursor_x, cursor_y) = buffer.get_cursor_location();
            let (cursor_x, cursor_y): (u16, u16) = (cursor_x.try_into()?, cursor_y.try_into()?);

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
            execute!(stdout(), MoveTo(cursor_x, cursor_y - scroll_y))?;
        }

        Ok(())
    }
}
