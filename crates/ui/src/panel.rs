use algebra::vec2::i16::I16Vec2;
use utils::key::Key;

use crate::widget::Widget;

pub trait Panel: Widget {
    fn on_active(&mut self) {}
    fn on_inactive(&mut self) {}
    fn on_keydown(&mut self, key: Key) {
        let _ = key;
    }
    fn on_scroll(&mut self, scroll: I16Vec2) {
        let _ = scroll;
    }
}
