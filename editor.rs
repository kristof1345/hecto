use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io;

mod terminal;
use terminal::{Position, Size, Terminal};

// globals
const VERSION: &str = env!("CARGO_PKG_VERSION"); // gets the version name from cargo.toml

pub struct Editor {
    should_quit: bool, // we don't have to declare that we want a mutatable field
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_events(&event);
        }
        Ok(())
    }

    fn evaluate_events(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
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

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), io::Error> {
        let width = Terminal::size()?.width as usize;
        let name_and_version = format!("Hecto -- version {}", VERSION);
        let len = name_and_version.len();
        let padding = ((width.saturating_sub(len)) / 2); // saturating_sub subtracts a number from a number without underflowing it
        let spaces = " ".repeat(padding.satuating_sub(1));
        Terminal::print(format!("~{padding}{name_and_version}").as_str())?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;
            if height / 3 == row {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                // saturating_add adds a number to a number without overflowing it
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
