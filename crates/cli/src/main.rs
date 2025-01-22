use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossterm::event;
use editor::Editor;
use utils::event::Event;
use utils::term::init_term;
use utils::window::Window;

fn main() -> Result<()> {
    init_term()?;

    let editor = Arc::new(Mutex::new(Editor::new()?));

    let editor_clone = Arc::clone(&editor);
    thread::spawn(move || loop {
        let editor = editor_clone.lock().unwrap();
        editor.draw().unwrap();
        thread::sleep(Duration::from_millis(16));
    });

    loop {
        let event = Event::CrosstermEvent(event::read()?);
        {
            let mut editor = editor.lock().unwrap();
            editor.on_event(event)?;
            editor.draw()?;
        }
    }
}
