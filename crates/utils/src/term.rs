use std::{io::stdout, process::exit};

use anyhow::Result;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

pub fn init_term() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

pub fn clean_term() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn get_term_size() -> Result<(u16, u16)> {
    Ok(size()?)
}

pub fn safe_exit() {
    clean_term().unwrap();
    exit(0);
}
