mod terminal;

use std::io::{stdout, Error};

use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
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
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
