pub(crate) mod buf;
pub(crate) mod cursor;

use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use buf::EditorBuffer;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use utils::{
    rect::Rect,
    term::{get_term_size, safe_exit},
};

pub struct Editor {
    rect: Rect,
    buffers: Vec<EditorBuffer>,
    open_buffer: Option<usize>,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            rect: Rect::new(0, 0, term_w, term_h),
            buffers: Vec::new(),
            open_buffer: None,
        })
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        let (term_w, term_h) = get_term_size()?;

        Ok(Self {
            rect: Rect::new(0, 0, term_w, term_h),
            buffers: vec![EditorBuffer::open(path)?],
            open_buffer: Some(0),
        })
    }

    pub fn on_event(&mut self, evt: Event) -> Result<()> {
        let (term_w, term_h) = get_term_size()?;
        self.rect.w = term_w;
        self.rect.h = term_h;

        match evt {
            Event::Key(evt) => match evt.code {
                KeyCode::Char('q') => safe_exit(),
                _ => {}
            },
            _ => {}
        }

        if let Some(open_buffer) = self.open_buffer {
            self.buffers[open_buffer].on_event(evt)?;
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        if let Some(open_buffer) = self.open_buffer {
            let (cursor_x, cursor_y) = self.buffers[open_buffer].get_cursor_location();
            let (cursor_x, cursor_y) = (cursor_x.try_into()?, cursor_y.try_into()?);

            let (_scroll_x, scroll_y) = self.buffers[open_buffer].get_scroll_location();
            let lines = self.buffers[open_buffer].get_lines();

            for (index, line) in lines
                .iter()
                .skip(scroll_y)
                .take(self.rect.h.into())
                .enumerate()
            {
                let y: u16 = index.try_into()?;
                execute!(stdout(), MoveTo(self.rect.x, self.rect.y + y), Print(line))?;
            }

            execute!(stdout(), MoveTo(cursor_x, cursor_y))?;
        }

        Ok(())
    }
}
