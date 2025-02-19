use std::io;

use clap::Parser;
use fluent_templates::static_loader;
use tui_studio::{
    utils::term::{init_term, safe_exit},
    App,
};

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
    path: Vec<String>,
}

fn main() -> io::Result<()> {
    init_term()?;

    let args = Args::parse();
    App::run(args.path);

    safe_exit();
    Ok(())
}
