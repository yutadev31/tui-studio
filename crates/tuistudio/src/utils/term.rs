use std::{
    io::{self, stdout},
    process::exit,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

pub fn init_term() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

pub fn clean_term() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorStyle::DefaultUserShape
    )?;
    Ok(())
}

pub fn get_term_size() -> io::Result<(u16, u16)> {
    size()
}

pub fn safe_exit() {
    clean_term().unwrap();
    exit(0);
}
