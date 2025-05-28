use std::io::{stdout, Error};

use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn initialize() -> Result<(), Error> {
        _ = enable_raw_mode();
        Self::clear_screen()
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn draw_rows() -> Result<(), Error> {
        let mut stdout = stdout();
        let (columns, rows) = size()?;

        for row in 1..rows {
            execute!(stdout, MoveTo(1, row), Print("~"));
        }

        execute!(stdout, MoveTo(1, 1));

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
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
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
