use crate::domain::transformer::LittleTransformer;

pub struct Point {
    pub id: u64,
    pub name: String,
    pub transformer: LittleTransformer
}

impl Point {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: format!("Point {}", id),
            transformer: LittleTransformer::new(),
        }
    }
    
    pub fn transform(&mut self, position: (f64, f64, f64)) {
        self.transformer.position = position;
    }
    
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
