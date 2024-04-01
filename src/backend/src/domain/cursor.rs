use crate::domain::mesh::Mesh;
use crate::domain::transformer::LittleTransformer;

pub struct Cursor {
    pub name: String,
    pub transformer: LittleTransformer,
    pub mesh: Mesh,
}

impl Cursor {
    pub fn new() -> Self {
        let transformer = LittleTransformer::new();
        let mesh = Mesh::from_cursor();

        Self {
            name: "Cursor".to_string(),
            transformer,
            mesh,
        }
    }

    pub fn transform(&mut self, position: (f64, f64, f64)) {
        self.transformer.position = position;
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
