use std::{cmp, ops::Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

enum GraphemeWidth {
    Half,
    Full,
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(text: &str) -> Self {
        let fragments = text
            .graphemes(true)
            .map(|grapheme| {
                let width = grapheme.widht();
                let rendered_width = match width {
                    0 | 1 => GraphemeWidth::Hald,
                    _ => GraphemeWidth::Full,
                };

                let replacement = match width {
                    0 => Some('.'),
                    _ => None,
                };

                TextFragment {
                    grapheme,
                    rendered_width,
                    replacement,
                }
            })
            .collect();

        Self { fragments }
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
