use std::collections::HashMap;

use crossterm::event::Event;

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn on_event(&self, evt: Event) {}
}
