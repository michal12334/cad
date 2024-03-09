use std::ops::Range;
use std::str::FromStr;
use std::string::ToString;
use egui::TextBuffer;

pub struct TypedTextBuffer<T: FromStr + ToString + Copy> {
    inner: String,
    default_value: T,
}

impl<T: FromStr + ToString + Copy> TypedTextBuffer<T> {
    pub fn new(default_value: T) -> Self {
        Self {
            inner: default_value.to_string(),
            default_value,
        }
    }
    
    pub fn value(&self) -> T {
        let default_value = self.default_value;
        match T::from_str(&self.inner) {
            Ok(value) => { value }
            Err(_) => { default_value }
        }
    }
}

impl<T: FromStr + ToString + Copy> TextBuffer for TypedTextBuffer<T> {
    fn is_mutable(&self) -> bool {
        return true;
    }

    fn as_str(&self) -> &str {
        return &self.inner;
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let mut new_text = self.inner.clone();
        new_text.insert_text(text, char_index);
        return match T::from_str(new_text.as_str()) {
            Ok(_) => { self.inner.insert_text(text, char_index) }
            Err(_) => { 0 }
        };
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.inner.delete_char_range(char_range);
    }
}
