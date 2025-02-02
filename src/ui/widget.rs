use algebra::vec2::u16::U16Vec2;

use super::renderer::WidgetRenderer;

pub trait Widget: Send {
    fn render(&self, renderer: &mut WidgetRenderer, size: U16Vec2);
}
