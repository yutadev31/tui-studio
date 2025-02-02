use std::sync::{Arc, Mutex};

use algebra::vec2::u16::U16Vec2;

use crate::editor::Editor;

use super::{renderer::UIRenderer, widget::Widget};

pub struct InfoBar {
    editor: Arc<Mutex<Editor>>,
}

impl InfoBar {
    pub fn new(editor: Arc<Mutex<Editor>>) -> Self {
        Self { editor }
    }
}

impl Widget for InfoBar {
    fn render(&self, renderer: &mut UIRenderer, size: U16Vec2) {
        let mut line = String::new();
        renderer.render_text(line, U16Vec2::new(0, 0));
    }
}
