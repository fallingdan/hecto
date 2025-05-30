use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

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
        queue!(stdout, Clear(ClearType::All))?;

        Terminal::flush()?;

        Ok(())
    }

    pub fn clear(clear_type: ClearType) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(clear_type))?;

        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn get_size() -> Result<Size, Error> {
        let (x, y) = size()?;

        Ok(Size {
            width: x,
            height: y,
        })
    }

    pub fn print(text: &str) -> Result<(), Error> {
        queue!(stdout(), Print(text))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        execute!(stdout(), Hide)?;

        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        execute!(stdout(), Show)?;

        Ok(())
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
}
