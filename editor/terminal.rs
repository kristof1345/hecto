use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{self, stdout};

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        // execute!(stdout(), Clear(ClearType::Purge)?; // don't delete - clears history - only commented it out for debug purposes
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), io::Error> {
        size()
    }
}
