use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use arboard::Clipboard;
use unicode_width::UnicodeWidthChar;

use crate::utils::{
    event::Event,
    key_binding::Key,
    vec2::{IVec2, UVec2},
};

use super::{
    action::{EditorBufferAction, EditorCursorAction, EditorEditAction},
    mode::EditorMode,
};

#[derive(Default)]
pub(crate) struct EditorBuffer {
    path: Option<PathBuf>,
    file: Option<File>,
    content: Vec<String>,
    cursor: UVec2,
    scroll: UVec2,
}

impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            content: vec![String::new(), String::new()],
            ..Default::default()
        }
    }

    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let mut buf = String::new();

        let mut file = Self::open_file(&path)?;
        file.read_to_string(&mut buf)
            .context("Failed to read file")?;

        Ok(Self {
            path: Some(path),
            file: Some(file),
            content: buf.lines().map(|line| line.to_string()).collect(),
            ..Default::default()
        })
    }

    pub fn open_file(path: &PathBuf) -> anyhow::Result<File> {
        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .context("Failed to open file")?)
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let Some(path) = &self.path else {
            return Err(anyhow!("No buffer open"));
        };

        if let None = self.file {
            let file = Self::open_file(path)?;
            self.file = Some(file);
        }

        let Some(file) = &mut self.file else {
            return Err(anyhow!("Failed to get file"));
        };

        file.seek(SeekFrom::Start(0))
            .context("Failed to seek file")?;
        file.write_all(self.content.join("\n").as_bytes())
            .context("Failed to write file")?;

        Ok(())
    }

    pub fn get_line_count(&self) -> usize {
        self.content.len()
    }

    pub fn get_line_length(&self, y: usize) -> usize {
        self.content[y].chars().count()
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.content.clone()
    }

    pub fn get_line(&self, y: usize) -> String {
        self.content[y].clone()
    }

    pub fn insert_line(&mut self, y: usize) {
        self.content.insert(y, String::new());
    }

    pub fn delete_line(&mut self, y: usize) {
        self.content.remove(y);
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.content[y].clone();
        let (p0, p1) = original.split_at(x);
        self.content[y] = p0.to_string();
        self.content.insert(y + 1, p1.to_string());
    }

    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.content.len() {
            let combined = self.content[y].clone() + &self.content[y + 1].clone();
            self.content[y] = combined;
            self.content.remove(y + 1);
        }
    }

    pub fn insert_char(&mut self, x: usize, y: usize, ch: char) {
        self.content[y].insert(x, ch);
    }

    pub fn delete_char(&mut self, x: usize, y: usize) {
        self.content[y].remove(x);
    }

    pub fn delete_key(&mut self, mode: &EditorMode) {
        let cursor = self.get_position(mode);

        if cursor.x == self.get_line_length(cursor.y) {
            self.join_lines(cursor.y);
        } else {
            self.delete_char(cursor.x, cursor.y);
        }
    }

    pub fn backspace_key(&mut self, mode: &EditorMode, window_size: UVec2) -> anyhow::Result<()> {
        let cursor = self.get_position(mode);

        if cursor.x == 0 {
            if cursor.y == 0 {
                return Ok(());
            }

            let line_length = self.get_line_length(cursor.y - 1);
            self.move_by_y(-1, mode, window_size);

            // line_length - 1 するのが本来は良いが usize が 0 以下になるのを防ぐため、- 1 はしない
            self.move_to_x(line_length);
            self.join_lines(cursor.y - 1);
        } else {
            let remove_x = cursor.x - 1;

            self.move_by_x(-1, mode);
            self.delete_char(remove_x, cursor.y);
        }

        Ok(())
    }

    pub fn get_position(&self, mode: &EditorMode) -> UVec2 {
        UVec2::new(self.clamp_x(self.cursor.x, mode), self.cursor.y)
    }

    pub fn get_draw_position(&self, mode: &EditorMode) -> UVec2 {
        let line = self.get_line(self.cursor.y);
        let x = self.clamp_x(self.cursor.x, mode);

        UVec2::new(
            line.to_string()
                .chars()
                .take(x)
                .filter_map(|c| c.width())
                .fold(0, |sum, x| sum + x),
            self.cursor.y,
        )
    }

    fn clamp_x(&self, x: usize, mode: &EditorMode) -> usize {
        let line_len = self.get_line_length(self.cursor.y);

        match mode {
            EditorMode::Normal | EditorMode::Visual { .. } => {
                if line_len == 0 {
                    0
                } else if x > line_len - 1 {
                    line_len - 1
                } else {
                    x
                }
            }
            EditorMode::Insert { .. } => {
                if x > line_len {
                    line_len
                } else {
                    x
                }
            }
            _ => x,
        }
    }

    fn clamp_y(&self, y: usize) -> usize {
        let line_count = self.get_line_count();

        if y > line_count - 1 {
            line_count - 1
        } else {
            y
        }
    }

    pub fn move_to_x(&mut self, x: usize) {
        self.cursor.x = x;
    }

    pub fn move_to_y(&mut self, y: usize, mode: &EditorMode, window_size: UVec2) {
        self.cursor.y = self.clamp_y(y);
        self.sync_scroll_y(mode, window_size);
    }

    pub fn move_to(&mut self, target: UVec2, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(target.y, mode, window_size);
        self.move_to_x(target.x);
    }

    pub fn move_to_top(&mut self, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(0, mode, window_size);
    }

    pub fn move_to_bottom(&mut self, mode: &EditorMode, window_size: UVec2) {
        self.move_to_y(self.get_line_count() - 1, mode, window_size)
    }

    pub fn move_to_back_word(&mut self) {
        let line = self.get_line(self.cursor.y);
        let mut x = self.cursor.x;

        if x == 0 {
            if self.cursor.y == 0 {
                return;
            }

            self.cursor.y -= 1;
            x = self.get_line_length(self.cursor.y);
        }

        while x > 0 {
            let Some(c) = line.to_string().chars().nth(x - 1) else {
                x -= 1;
                continue;
            };

            if c.is_whitespace() && self.cursor.x != x {
                break;
            }

            x -= 1;
        }

        self.cursor.x = x;
    }

    pub fn move_to_next_word(&mut self) {
        let line = self.get_line(self.cursor.y);
        let mut x = self.cursor.x;

        if x == self.get_line_length(self.cursor.y) {
            if self.cursor.y == self.get_line_count() - 1 {
                return;
            }

            self.cursor.y += 1;
            self.cursor.x = 0;
            return;
        }

        while x < self.get_line_length(self.cursor.y) {
            let c = line.to_string().chars().nth(x).unwrap();
            if c.is_whitespace() && x != self.cursor.x {
                break;
            }

            x += 1;
        }

        self.cursor.x = x;
    }

    pub fn move_by_x(&mut self, x: isize, mode: &EditorMode) {
        if x > 0 {
            self.sync(mode);
            self.cursor.x = self.clamp_x(self.cursor.x + x as usize, mode);
        } else if x < 0 {
            self.sync(mode);
            if self.cursor.x < -x as usize {
                self.cursor.x = 0;
            } else {
                self.cursor.x -= -x as usize;
            }
        }
    }

    pub fn move_by_y(&mut self, y: isize, mode: &EditorMode, window_size: UVec2) {
        if y > 0 {
            self.cursor.y = self.clamp_y(self.cursor.y + y as usize);
        } else if y < 0 {
            if self.cursor.y < -y as usize {
                self.cursor.y = 0;
            } else {
                self.cursor.y -= -y as usize;
            }
        }

        self.sync_scroll_y(mode, window_size);
    }

    pub fn move_by(&mut self, offset: IVec2, mode: &EditorMode, window_size: UVec2) {
        self.move_by_x(offset.x, mode);
        self.move_by_y(offset.y, mode, window_size);
    }

    pub fn sync_x(&mut self, mode: &EditorMode) {
        self.cursor.x = self.clamp_x(self.cursor.x, mode);
    }

    pub fn sync_y(&mut self) {
        self.cursor.y = self.clamp_y(self.cursor.y);
    }

    pub fn sync(&mut self, mode: &EditorMode) {
        self.sync_x(mode);
        self.sync_y();
    }

    pub fn get_offset(&self) -> UVec2 {
        self.scroll.clone()
    }

    pub fn scroll_to_x(&mut self, x: usize) {
        self.scroll.x = x;
    }

    pub fn scroll_to_y(&mut self, y: usize) {
        self.scroll.y = y;
    }

    pub fn sync_scroll_y(&mut self, mode: &EditorMode, window_size: UVec2) {
        let cursor = self.get_position(mode);

        if cursor.y >= self.scroll.y + window_size.y - 1 {
            self.scroll_to_y(cursor.y - (window_size.y - 1));
        } else if cursor.y < self.scroll.y {
            self.scroll_to_y(cursor.y);
        }
    }

    pub fn on_action(
        &mut self,
        action: EditorBufferAction,
        mode: &EditorMode,
        _clipboard: &mut Option<Clipboard>,
        window_size: UVec2,
    ) -> anyhow::Result<()> {
        match action {
            EditorBufferAction::Save => self.save()?,
            EditorBufferAction::Cursor(action) => match action {
                EditorCursorAction::Left => self.move_by_x(-1, mode),
                EditorCursorAction::Down => self.move_by_y(1, mode, window_size),
                EditorCursorAction::Up => self.move_by_y(-1, mode, window_size),
                EditorCursorAction::Right => self.move_by_x(1, mode),
                EditorCursorAction::LineStart => self.move_to_x(0),
                EditorCursorAction::LineEnd => self.move_to_x(usize::MAX),
                EditorCursorAction::Top => self.move_to_top(mode, window_size),
                EditorCursorAction::Bottom => self.move_to_bottom(mode, window_size),
                EditorCursorAction::NextWord => self.move_to_next_word(),
                EditorCursorAction::BackWord => self.move_to_back_word(),
            },
            EditorBufferAction::Edit(action) => match action {
                EditorEditAction::DeleteLine => self.delete_line(self.cursor.y),
                // EditorEditAction::DeleteSelection => {
                //     self.delete_selection(cursor, mode, clipboard)?
                // }
                // EditorEditAction::YankLine => self.yank_line(cursor, mode, clipboard)?,
                // EditorEditAction::YankSelection => self.yank_selection(cursor, mode, clipboard)?,
                // EditorEditAction::Paste => self.paste(cursor, mode, clipboard, window_size)?,
                _ => {}
            },
        };

        Ok(())
    }

    pub fn on_event(
        &mut self,
        evt: Event,
        mode: &EditorMode,
        window_size: UVec2,
    ) -> anyhow::Result<Option<EditorMode>> {
        let cursor_pos = self.get_position(mode);
        let cursor_x = cursor_pos.x;
        let cursor_y = cursor_pos.y;

        match mode {
            EditorMode::Normal => match evt {
                Event::Click(pos) => {
                    let num_len = (self.get_line_count() - 1).to_string().len();
                    let offset_x = num_len + 1;
                    let scroll_y = self.get_offset().y;

                    let x = if let Some(x) = pos.x.checked_sub(offset_x) {
                        x
                    } else {
                        0
                    };

                    self.move_to_y(pos.y + scroll_y, mode, window_size);
                    self.move_to_x(x);
                }
                // Event::Scroll(scroll) => self.scroll_by(scroll, &self.code),
                _ => {}
            },
            EditorMode::Insert { append: _ } => match evt {
                Event::Input(key) => match key {
                    Key::Delete => self.delete_key(mode),
                    Key::Backspace => self.backspace_key(mode, window_size)?,
                    Key::Char('\t') => {
                        self.insert_char(cursor_x, cursor_y, '\t');
                        self.move_by_x(1, mode);
                    }
                    Key::Char('\n') => {
                        self.split_line(cursor_x, cursor_y);
                        self.move_by_y(1, mode, window_size);
                        self.move_to_x(0);
                    }
                    Key::Char(c) => {
                        self.insert_char(cursor_x, cursor_y, c);
                        self.move_by_x(1, mode);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        Ok(None)
    }
}

impl ToString for EditorBuffer {
    fn to_string(&self) -> String {
        self.content.join("\n")
    }
}
