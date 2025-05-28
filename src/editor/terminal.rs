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
        Self::clear_screen()?;
        Self::draw_rows()
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn draw_rows() -> Result<(), Error> {
        let mut stdout = stdout();
        let (_, rows) = size()?;

        for row in 1..rows {
            execute!(stdout, MoveTo(1, row), Print("~"))?;
        }

        execute!(stdout, MoveTo(1, 1))?;

        Ok(())
    }
}
