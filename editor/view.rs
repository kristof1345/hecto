use super::terminal;
use std::io;
use terminal::{Size, Terminal};

mod buffer;
use buffer::Buffer;

const VERSION: &str = env!("CARGO_PKG_VERSION"); // gets the version name from cargo.toml

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), io::Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }

        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(row) {
                Terminal::print(line)?;
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

    pub fn render_welcome_screen() -> Result<(), io::Error> {
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

    pub fn load(&mut self, filename: &String) {
        if let Ok(lines) = Buffer::load(filename) {
            self.buffer = lines;
        }
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
