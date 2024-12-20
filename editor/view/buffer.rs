use std::fs;
use std::io;
use std::vec::Vec;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(filename: &str) -> Result<Self, io::Error> {
        let file_contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        // let lines = vec![file_contents.to_string()];
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
