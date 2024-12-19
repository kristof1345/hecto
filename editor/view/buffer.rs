use std::vec::Vec;

pub struct Buffer {
    pub lines: Vec<String>,
    pub is_empty: bool,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            is_empty: true,
        }
    }
}
