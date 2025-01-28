mod app;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::event;
use fluent_templates::static_loader;
use tokio::sync::Mutex;
use utils::component::{Component, DrawableComponent};
use utils::event::Event;
use utils::term::init_term;

static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: Option<String>,
}

async fn run_app(path: Option<String>) -> Result<()> {
    let app = Arc::new(Mutex::new(App::new(path)?));
    app.lock().await.init();

    let app_clone = Arc::clone(&app);
    tokio::spawn(async move {
        loop {
            {
                let app = app_clone.lock().await;
                app.draw().await.unwrap();
            }
            thread::sleep(Duration::from_millis(32));
        }
    });

    {
        let mut app = app.lock().await;
        app.on_event(Event::Resize).await?;
    }

    loop {
        let event = Event::CrosstermEvent(event::read()?);
        {
            let mut app = app.lock().await;
            app.on_event(event).await?;
            app.draw().await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init_term()?;
    let args = Args::parse();

    run_app(args.path).await?;

    Ok(())
}
