use crossterm::event::{read, Event, KeyEvent, KeyEventKind};

use std::io;
use std::panic::{set_hook, take_hook};
mod terminal;
use terminal::Terminal;

mod view;
use view::View;

mod editorcommand;
use editorcommand::EditorCommand;

#[derive(Default)]
pub struct Editor {
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
            // location: Location::default(),
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
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not process command: {err:?}")
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();

        let _ = Terminal::move_caret_to(self.view.get_position());

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
