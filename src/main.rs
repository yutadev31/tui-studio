use clap::Parser;
use log::error;
use tui_studio::{
    utils::term::{init_term, safe_exit},
    App,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    init_term()?;
    let args = Args::parse();

    match App::run(args.path) {
        Err(err) => error!("{}", err),
        Ok(_) => {}
    };

    safe_exit();
    Ok(())
}
