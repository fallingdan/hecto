use std::io::{stdout, Error};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        _ = enable_raw_mode();
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn move_cursor_to(column: u16, row: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(column, row))?;
        Ok(())
    }

    pub fn get_size() -> Result<(u16, u16), Error> {
        size()
    }

    pub fn print(text: &str) -> Result<(), Error> {
        execute!(stdout(), Print(text))?;
        Ok(())
    }
}
