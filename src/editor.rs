mod terminal;

use std::io::Error;

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};

use crossterm::terminal::ClearType;
use terminal::{Position, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    startup_complete: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            startup_complete: false,
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            if !self.startup_complete {
                Editor::display_welcome()?;
                self.startup_complete = true;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 });
            Terminal::print("Goodbye!")?;
        } else {
            Editor::draw_rows()?;
        }
        Terminal::flush()?;

        Terminal::show_cursor()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let size = Terminal::get_size()?;

        for row in 0..size.height {
            Terminal::move_cursor_to(Position { x: 0, y: row })?;
            Terminal::clear(ClearType::CurrentLine)?;
            Terminal::print("~")?;
        }
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        Terminal::flush()?;

        Ok(())
    }

    /// Displays the welcome message to Hecto
    /// If the terminal width is too small, message will not display
    fn display_welcome() -> Result<(), Error> {
        let size = Terminal::get_size()?;
        let target_row = size.height / 3;
        let welcome_message = format!("{NAME} -- {VERSION}");
        let welcome_length = welcome_message.len() as u16;

        if size.width < welcome_length {
            return Ok(());
        }

        let message_start_position = (size.width - welcome_length) / 2;

        Terminal::move_cursor_to(Position {
            x: message_start_position - 1,
            y: target_row,
        })?;
        Terminal::print(&welcome_message)?;
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        Terminal::flush()?;

        Ok(())
    }
}
