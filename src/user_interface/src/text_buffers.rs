use std::ops::Range;
use std::str::FromStr;
use egui::TextBuffer;

pub struct F32TextBuffer {
    inner: String,
}

impl F32TextBuffer {
    pub fn new() -> Self {
        Self {
            inner: "0.0".to_string(),
        }
    }
}

impl TextBuffer for F32TextBuffer {
    fn is_mutable(&self) -> bool {
        return true;
    }

    fn as_str(&self) -> &str {
        return &self.inner;
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let mut new_text = self.inner.clone();
        new_text.insert_text(text, char_index);
        return match f32::from_str(new_text.as_str()) {
            Ok(_) => { self.inner.insert_text(text, char_index) }
            Err(_) => { 0 }
        };
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.inner.delete_char_range(char_range);
    }
}
