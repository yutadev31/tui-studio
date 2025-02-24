use std::sync::{Arc, Mutex, MutexGuard};

use algebra::vec2::{isize::ISizeVec2, usize::USizeVec2};
use utils::wide_string::WideString;

use crate::{
    core::buf::EditorBuffer,
    types::error::{EditorError, EditorResult},
};

#[derive(Debug, Clone)]
pub struct EditorBufferAPI {
    buf: Arc<Mutex<EditorBuffer>>,
}

impl EditorBufferAPI {
    fn get_buf(&self) -> EditorResult<MutexGuard<EditorBuffer>> {
        self.buf.lock().map_err(|_| EditorError::MutexUnlockFailed)
    }

    pub(crate) fn new(buf: Arc<Mutex<EditorBuffer>>) -> Self {
        Self { buf }
    }

    /* Content */
    pub fn get_content_string(&self) -> EditorResult<String> {
        let buf = self.get_buf()?;
        let content = buf.get_content();
        Ok(content.to_string())
    }

    pub fn get_content_line(&self, y: usize) -> EditorResult<WideString> {
        let buf = self.get_buf()?;
        let content = buf.get_content();
        Ok(content.get_line(y)?)
    }

    pub fn get_content_all_lines(&self) -> EditorResult<Vec<WideString>> {
        let buf = self.get_buf()?;
        let content = buf.get_content();
        Ok(content.get_all_lines())
    }

    pub fn set_content_string(&mut self, new_content: String) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let content = buf.get_content_mut();
        content.set_string(new_content);
        Ok(())
    }

    pub fn set_content_lines(&mut self, new_content: Vec<WideString>) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let content = buf.get_content_mut();
        content.set_lines(new_content);
        Ok(())
    }

    pub fn insert_char(&mut self, ch: char) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let pos = buf.get_cursor().get();
        let content = buf.get_content_mut();
        content.insert_char(ch, pos);
        Ok(())
    }

    pub fn delete(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let pos = buf.get_cursor().get();
        let content = buf.get_content_mut();
        content.delete_char(pos);
        Ok(())
    }

    pub fn backspace(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let pos = buf.get_cursor().get();
        let content = buf.get_content_mut();
        content.delete_char(USizeVec2 {
            x: match pos.x.checked_sub(1) {
                Some(x) => x,
                None => pos.x,
            },
            y: pos.y,
        });
        Ok(())
    }

    /* Scroll */
    pub fn get_scroll_offset(&self) -> EditorResult<USizeVec2> {
        let buf = self.get_buf()?;
        let scroll = buf.get_scroll();
        Ok(scroll.get())
    }

    /* Cursor */
    pub fn get_cursor_position(&self) -> EditorResult<USizeVec2> {
        let buf = self.get_buf()?;
        let cursor = buf.get_cursor();
        Ok(cursor.get())
    }

    pub fn get_cursor_draw_position(&self) -> EditorResult<USizeVec2> {
        let buf = self.get_buf()?;
        let content = buf.get_content();
        let cursor = buf.get_cursor();
        Ok(cursor.get_draw_position(content)?)
    }

    pub fn move_cursor_to(&mut self, target: USizeVec2) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to(target);
        Ok(())
    }

    pub fn move_cursor_to_x(&mut self, x: usize) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to_x(x);
        Ok(())
    }

    pub fn move_cursor_to_y(&mut self, y: usize) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to_y(y);
        Ok(())
    }

    pub fn move_cursor_to_top(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to_top();
        Ok(())
    }

    pub fn move_cursor_to_bottom(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let line_count = buf.get_content().get_line_count();
        let cursor = buf.get_cursor_mut();
        cursor.move_to_bottom(line_count);
        Ok(())
    }

    pub fn move_cursor_to_line_start(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to_line_start();
        Ok(())
    }

    pub fn move_cursor_to_line_end(&mut self) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor_y = buf.get_cursor().get().y;
        let line_length = buf.get_content().get_line_length(cursor_y)?;
        let cursor = buf.get_cursor_mut();
        cursor.move_to_line_end(line_length);
        Ok(())
    }

    pub fn move_cursor_by(&mut self, offset: ISizeVec2) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let (line_count, line_length) = {
            let cursor_y = buf.get_cursor().get().y;
            let content = buf.get_content();
            (content.get_line_count(), content.get_line_length(cursor_y)?)
        };
        let cursor = buf.get_cursor_mut();
        cursor.move_by(offset, line_length, line_count);
        Ok(())
    }

    pub fn move_cursor_by_x(&mut self, x: isize) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let line_count = buf.get_content().get_line_count();
        let cursor = buf.get_cursor_mut();
        cursor.move_by_x(x, line_count);
        Ok(())
    }

    pub fn move_cursor_by_y(&mut self, y: isize) -> EditorResult<()> {
        let mut buf = self.get_buf()?;
        let cursor_y = buf.get_cursor().get().y;
        let line_length = buf.get_content().get_line_length(cursor_y)?;
        let cursor = buf.get_cursor_mut();
        cursor.move_by_y(y, line_length);
        Ok(())
    }
}
