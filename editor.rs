use crossterm::event::KeyCode;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind, KeyModifiers};

use std::io;
use std::panic::{set_hook, take_hook};
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
    pub fn new() -> Result<Self, io::Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(first_arg) = args.get(1) {
            // do someting with filename
            view.load(first_arg);
        }
        Ok(Self {
            location: Location::default(),
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_events(event),
                Err(error) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {error:?}");
                    }
                }
            }
        }
    }

    fn evaluate_events(&mut self, event: Event) {
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
                    self.move_point(code);
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
    }

    fn move_point(&mut self, code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size().unwrap_or_default();

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
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::flush();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
