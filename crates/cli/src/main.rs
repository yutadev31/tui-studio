use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use crossterm::event;
use editor::Editor;
use utils::term::init_term;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: Option<String>,
}

fn main() -> Result<()> {
    init_term()?;

    let args = Args::parse();
    let mut editor = match args.path {
        None => Editor::new()?,
        Some(path) => Editor::open(PathBuf::from(path))?,
    };

    editor.draw()?;

    loop {
        editor.on_event(event::read()?)?;
        editor.draw()?;
    }
}
