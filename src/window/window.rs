use crate::utils::vec2::Vec2;

pub trait Window {
    fn render(&self, screen: &mut Box<[String]>, window_size: Vec2);
}

pub type BoxWindow = Box<dyn Window>;
