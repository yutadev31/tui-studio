use thiserror::Error;

use crate::utils::vec2::Vec2;

use super::window::BoxWindow;

#[derive(Debug, Error)]
pub enum WindowManagerError {
    #[error("Failed to get active window")]
    GetActiveFailed,
}

#[derive(Default)]
pub struct WindowManager {
    windows: Vec<BoxWindow>,
    active: usize,
}

impl WindowManager {
    pub fn draw(&self) -> Result<(), WindowManagerError> {
        let window = self
            .windows
            .get(self.active)
            .ok_or_else(|| WindowManagerError::GetActiveFailed)?;

        let mut screen =
            vec![String::new(), String::new(), String::new(), String::new()].into_boxed_slice();

        let height = screen.len();
        window.render(&mut screen, Vec2::new(4, height));

        Ok(())
    }
}
