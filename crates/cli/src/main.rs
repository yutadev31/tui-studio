mod app;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{panic, thread};

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::event;
use fluent_templates::static_loader;
use utils::component::{Component, DrawableComponent};
use utils::event::Event;
use utils::term::{init_term, safe_exit};

static_loader! {
    pub static LOCALES = {
        locales: "../../locales",
        fallback_language: "en-US",
    };
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: Option<String>,
}

fn run_app(path: Option<String>) -> Result<()> {
    let app = Arc::new(Mutex::new(App::new(path)?));
    app.lock().unwrap().init();

    let app_clone = Arc::clone(&app);
    thread::spawn(move || loop {
        {
            let app = app_clone.lock().unwrap();
            app.draw().unwrap();
        }
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

fn main() -> Result<()> {
    init_term()?;
    let args = Args::parse();

    run_app(args.path)?;

    Ok(())
}
