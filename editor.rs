use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{self, stdout};

pub struct Editor {
    should_quit: bool, // we don't have to declare that we want a mutatable field
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        Self::draw_rows().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            let event = read()?;
            self.evaluate_events(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
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
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), io::Error> {
        let (_cols, rows) = size().unwrap();
        let mut stdout = stdout();

        for row in 0..rows {
            execute!(stdout, MoveTo(0, row))?;
            print!("~");
        }

        execute!(stdout, MoveTo(0, 0))?;

        Ok(())
    }
}
