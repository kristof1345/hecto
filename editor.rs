use crossterm::event::KeyCode;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind, KeyModifiers};

use std::io;
mod terminal;
use terminal::{Position, Size, Terminal};

mod view;
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    location: Location,
    should_quit: bool, // we don't have to declare that we want a mutatable field
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn read_args(&mut self) -> Result<(), io::Error> {
        let args: Vec<String> = std::env::args().collect();
        if let Some(first_arg) = args.get(1) {
            // do someting with filename
            self.view.load(first_arg);
        }

        Ok(())
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        self.read_args().unwrap();
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_events(event)?;
        }
        Ok(())
    }

    fn evaluate_events(&mut self, event: Event) -> Result<(), io::Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right, _) => {
                    self.move_point(code)?;
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                let width = width_u16 as usize;
                let height = height_u16 as usize;

                self.view.resize(Size { height, width })
            }
            _ => {}
        }

        Ok(())
    }

    fn move_point(&mut self, code: KeyCode) -> Result<(), io::Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;

        match code {
            KeyCode::Up => {
                if y != 0 {
                    y = y.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if y < height - 1 {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x != 0 {
                    x = x.saturating_sub(1);
                }
            }
            KeyCode::Right => {
                if x < width - 1 {
                    x = x.saturating_add(1);
                }
            }
            _ => (),
        }

        self.location = Location { x, y };

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::flush()?;
        Ok(())
    }
}
