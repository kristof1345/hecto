use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};

use core::fmt::Display;
use std::io::{self, stdout, Write};

pub struct Terminal;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn terminate() -> Result<(), io::Error> {
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::flush()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        // queue!(stdout(), Clear(ClearType::Purge)?; // don't delete - clears history - only commented it out for debug purposes
        Ok(())
    }

    pub fn clear_line() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), io::Error> {
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, io::Error> {
        let (width_u16, height_u16) = size()?;
        let width = width_u16 as usize;
        let height = height_u16 as usize;

        Ok(Size { width, height })
    }

    pub fn hide_cursor() -> Result<(), io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print<T: Display>(string: T) -> Result<(), io::Error> {
        Self::queue_command(Print(string))?;
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
