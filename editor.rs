use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io;

mod terminal;
use terminal::Terminal;

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
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), io::Error> {
        let rows = Terminal::size()?.1; // ei the height

        for row in 0..rows {
            print!("~");
            if row + 1 < rows {
                print!("\r\n");
            }
        }

        Ok(())
    }
}
