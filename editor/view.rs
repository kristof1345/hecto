use super::terminal;
use std::str;
use terminal::{Size, Terminal};

mod buffer;
use buffer::Buffer;

const VERSION: &str = env!("CARGO_PKG_VERSION"); // gets the version name from cargo.toml

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;

        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, truncated_line);
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }

        self.needs_redraw = false;
    }

    pub fn load(&mut self, filename: &String) {
        if let Ok(lines) = Buffer::load(filename) {
            self.buffer = lines;
            self.needs_redraw = true;
        }
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let name_and_version = format!("Hecto -- version {}", VERSION);
        let len = name_and_version.len();
        if width <= len {
            return "~".to_string();
        }

        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2; // saturating_sub subtracts a number from a number without underflowing it

        let mut full_mess = format!("~{}{}", " ".repeat(padding), name_and_version);
        full_mess.truncate(width);
        full_mess
    }

    fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);

        debug_assert!(result.is_ok(), "Failed to render line");
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
