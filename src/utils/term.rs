use std::{
    io::{self, stdout},
    process::exit,
};

use crossterm::{
    cursor::{SetCursorStyle, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use super::{log::init_logger, vec2::UVec2};

pub fn init_term() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    init_logger().unwrap();
    Ok(())
}

pub fn clean_term() -> io::Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Show,
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorStyle::DefaultUserShape
    )?;
    disable_raw_mode()?;
    Ok(())
}

pub fn get_term_size() -> io::Result<UVec2> {
    let (w, h) = size()?;
    Ok(UVec2::new(w as usize, h as usize))
}

pub fn safe_exit() {
    clean_term().unwrap();
    exit(0);
}
