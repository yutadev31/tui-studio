use algebra::vec2::u16::U16Vec2;

use crate::ui::{renderer::UIRenderer, widget::Widget};

#[derive(Default)]
pub struct TabPanel {
    children: Vec<Box<dyn Widget>>,
    current_index: Option<usize>,
}

impl TabPanel {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            current_index: if children.is_empty() { None } else { Some(0) },
            children,
        }
    }
}

impl Widget for TabPanel {
    fn render(&self, renderer: &mut UIRenderer, size: U16Vec2) {
        if let Some(index) = self.current_index {
            if let Some(child) = self.children.get(index) {
                let child_renderer = UIRenderer::new(size);
                // child.draw(&mut child_screen, size);
                // for line in child_screen {
                //     screen.push(line);
                // }
            }
        }
    }
}
