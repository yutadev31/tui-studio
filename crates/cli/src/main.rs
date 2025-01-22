mod app;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use app::App;
use crossterm::event;
use utils::event::Event;
use utils::term::init_term;
use utils::window::Window;

fn main() -> Result<()> {
    init_term()?;

    let app = Arc::new(Mutex::new(App::new()?));

    let app_clone = Arc::clone(&app);
    thread::spawn(move || loop {
        let app = app_clone.lock().unwrap();
        app.draw().unwrap();
        thread::sleep(Duration::from_millis(16));
    });

    loop {
        let event = Event::CrosstermEvent(event::read()?);
        {
            let mut app = app.lock().unwrap();
            app.on_event(event)?;
            app.draw()?;
        }
    }
}
