use algebra::vec2::u16::U16Vec2;

use super::renderer::UIRenderer;

pub trait Widget: Send {
    fn render(&self, renderer: &mut UIRenderer, size: U16Vec2);
}
