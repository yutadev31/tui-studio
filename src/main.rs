use clap::Parser;
use fluent_templates::static_loader;
use tui_studio::{
    run_app,
    utils::term::{init_term, safe_exit},
    PublicAppError,
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
    path: Option<String>,
}

fn main() -> Result<(), PublicAppError> {
    init_term().map_err(|_| PublicAppError::Error)?;
    let args = Args::parse();

    run_app(args.path)?;

    safe_exit();

    Ok(())
}
