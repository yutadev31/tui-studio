use clap::Parser;
use fluent_templates::static_loader;
use tuistudio::{run_app, PublicAppError};
use utils::term::init_term;

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

fn main() -> Result<(), PublicAppError> {
    init_term().map_err(|_| PublicAppError::Error)?;
    let args = Args::parse();

    run_app(args.path)?;

    Ok(())
}
