use super::terminal;
use std::io;
use terminal::{Size, Terminal};

const VERSION: &str = env!("CARGO_PKG_VERSION"); // gets the version name from cargo.toml

#[derive(Default)]
pub struct View;

impl View {
    pub fn render() -> Result<(), io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;
            if height / 3 == row {
                Self::draw_welcome_message()?;
            } else {
                if row == 0 {
                    Self::draw_hello_world()?;
                } else {
                    Self::draw_empty_row()?;
                }
            }

            if row.saturating_add(1) < height {
                // saturating_add adds a number to a number without overflowing it
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_hello_world() -> Result<(), io::Error> {
        Terminal::print("Hello, world!")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), io::Error> {
        let width = Terminal::size()?.width as usize;
        let name_and_version = format!("Hecto -- version {}", VERSION);
        let len = name_and_version.len();
        let padding = (width.saturating_sub(len)) / 2; // saturating_sub subtracts a number from a number without underflowing it
        let spaces = " ".repeat(padding.saturating_sub(1));
        Terminal::print(format!("~{spaces}{name_and_version}").as_str())?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), io::Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
