use algebra::vec2::u16::U16Vec2;

use crate::renderer::Renderer;

pub trait Widget {
    fn draw(&self, renderer: &mut Renderer, size: U16Vec2);
    fn on_click(&mut self, pos: U16Vec2) {
        let _ = pos;
    }
}
