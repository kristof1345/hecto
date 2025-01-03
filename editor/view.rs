use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
use std::cmp::min;
use std::str;

mod buffer;
use buffer::Buffer;

mod location;
use location::Location;

mod line;

use self::line::Line;

const VERSION: &str = env!("CARGO_PKG_VERSION"); // gets the version name from cargo.toml

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
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

        let top = self.scroll_offset.y;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);

                Self::render_line(row, &line.get(left..right));
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        // let Size { height, .. } = self.size;

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::length);
                }
            }
            Direction::Right => {
                let width = self.buffer.lines.get(y).map_or(0, Line::length);
                if x < width {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
        }

        // snap cursor to end of line
        x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| min(x, line.length()));

        // snap cursor to last line
        y = min(y, self.buffer.lines.len());

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    pub fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y; // scroll backwards
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1); // sub height to get the 'y' we need and add 1
            offset_changed = true;
        }

        // Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        // whether we need to redraw or not
        self.needs_redraw = offset_changed;
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
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
