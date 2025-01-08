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
        let graphemes: Vec<&str> = self.string.graphemes(true).collect();
        let len = graphemes.len();

        // Clamp the range to ensure it's within bounds
        let start = range.start.min(len); // Prevent start from exceeding the length
        let end = range.end.min(len); // Prevent end from exceeding the length

        if start >= end {
            // If the range is invalid (start >= end), return an empty string
            return String::new();
        }

        graphemes[start..end].concat()
    }

    pub fn length(&self) -> usize {
        self.string.graphemes(true).count()
    }
}
