use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{queue, Command};

// use core::fmt::Display;
use std::io::{self, stdout, Write};

pub struct Terminal;

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Terminal {
    pub fn terminate() -> Result<(), io::Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        // Self::move_cursor_to(Position { col: 0, row: 0 })?;
        Self::flush()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), io::Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), io::Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        // Self::queue_command(Clear(ClearType::Purge))?; // don't delete - clears history - only commented it out for debug purposes
        Ok(())
    }

    pub fn clear_line() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), io::Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, io::Error> {
        let (width_u16, height_u16) = size()?;
        let width = width_u16 as usize;
        let height = height_u16 as usize;

        Ok(Size { width, height })
    }

    pub fn hide_caret() -> Result<(), io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), io::Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn print_row(row: usize, line: &str) -> Result<(), io::Error> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::clear_line()?;
        Self::print(line)?;
        Ok(())
    }

    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
