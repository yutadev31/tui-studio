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

use algebra::vec2::u16::U16Vec2;

use super::log::init_logger;

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

pub fn get_term_size() -> io::Result<U16Vec2> {
    Ok(U16Vec2::from(size()?))
}

pub fn safe_exit() {
    clean_term().unwrap();
    exit(0);
}
