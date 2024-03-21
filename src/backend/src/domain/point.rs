use crate::domain::transformer::LittleTransformer;

pub struct Point {
    pub id: u64,
    pub transformer: LittleTransformer
}

impl Point {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            transformer: LittleTransformer::new(),
        }
    }
    
    pub fn transform(&mut self, position: (f64, f64, f64)) {
        self.transformer.position = position;
    }
}
