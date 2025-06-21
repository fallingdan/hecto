mod terminal;

use std::io::Error;

use crossterm::event::KeyCode::{Down, Left, Right, Up};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyModifiers};

use crossterm::terminal::ClearType;
use terminal::{Position, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy)]
struct CursorPosition {
    row: u16,
    column: u16,
}

pub struct Editor {
    should_quit: bool,
    startup_complete: bool,
    cursor_position: CursorPosition,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            startup_complete: false,
            cursor_position: CursorPosition { row: 0, column: 0 },
        }
    }

    /// Starts the editor
    /// Handles initialization and teardown
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    /// Main Read Eval Print Loop
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

    /// Handle KeyEvents
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: _,
        }) = event
        {
            match code {
                Left | Right | Up | Down => self.handle_cursor_move(event),
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn handle_cursor_move(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers: _,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                Left => self
                    .update_cursor_position(CursorPosition {
                        column: self.cursor_position.column.saturating_sub(1),
                        ..self.cursor_position
                    })
                    .unwrap(),
                Right => self
                    .update_cursor_position(CursorPosition {
                        column: self.cursor_position.column.saturating_add(1),
                        ..self.cursor_position
                    })
                    .unwrap(),
                Up => self
                    .update_cursor_position(CursorPosition {
                        row: self.cursor_position.row.saturating_sub(1),
                        ..self.cursor_position
                    })
                    .unwrap(),
                Down => self
                    .update_cursor_position(CursorPosition {
                        row: self.cursor_position.row.saturating_add(1),
                        ..self.cursor_position
                    })
                    .unwrap(),
                _ => (),
            }
        }
    }

    fn update_cursor_position(&mut self, desired_position: CursorPosition) -> Result<(), Error> {
        let valid_move = self.valid_cursor_move(desired_position).unwrap_or(false);

        if valid_move {
            Terminal::move_cursor_to(Position {
                x: desired_position.column,
                y: desired_position.row,
            })?;

            self.cursor_position = desired_position;
        }

        Ok(())
    }

    /// Check if cursor move is within bounds before performing
    fn valid_cursor_move(&self, desired_position: CursorPosition) -> Result<bool, Error> {
        let terminal_size = Terminal::get_size()?;

        if desired_position.row >= terminal_size.height.into()
            || desired_position.column >= terminal_size.width.into()
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// Execute any queued operations and handle initial states
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            Terminal::print("Goodbye!")?;
        } else if !self.startup_complete {
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
