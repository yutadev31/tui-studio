use crate::utils::vec2::Vec2;

pub trait Window: Send {
    fn render(&self, screen: &mut Box<[String]>, window_size: Vec2);
}

pub(super) type BoxWindow = Box<dyn Window>;
