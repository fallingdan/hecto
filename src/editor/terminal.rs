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
    /// Initialization work to start a terminal instance
    pub fn initialize() -> Result<(), Error> {
        _ = enable_raw_mode();
        Self::clear_screen()
    }

    /// Teardown work to terminate the terminal instance
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    /// Clears the screen immediately
    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;

        Terminal::flush()?;

        Ok(())
    }

    /// Queues clearing of the entire screen
    pub fn clear(clear_type: ClearType) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(clear_type))?;

        Ok(())
    }

    /// Queues cusor position change
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    /// Returns the size of the screen
    pub fn get_size() -> Result<Size, Error> {
        let (x, y) = size()?;

        Ok(Size {
            width: x,
            height: y,
        })
    }

    /// Queues string to be printed at current cursor position
    pub fn print(text: &str) -> Result<(), Error> {
        queue!(stdout(), Print(text))?;
        Ok(())
    }

    /// Hides cursor immediatly
    pub fn hide_cursor() -> Result<(), Error> {
        execute!(stdout(), Hide)?;

        Ok(())
    }

    /// Shows cursor immediately
    pub fn show_cursor() -> Result<(), Error> {
        execute!(stdout(), Show)?;

        Ok(())
    }

    /// Flushes stdout buffer to print any queued actions
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
}
