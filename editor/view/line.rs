use std::{cmp, ops::Range};

use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(text: &str) -> Self {
        Self {
            string: String::from(text),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn length(&self) -> usize {
        let grapehemed_str = self.string.graphemes(true).collect::<Vec<&str>>();
        grapehemed_str.len()
    }
}
