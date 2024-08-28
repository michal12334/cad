#[derive(Debug, Clone)]
pub struct IdGenerator {
    pub next_id: u64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn next(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn set_next_id(&mut self, next_id: u64) {
        self.next_id = next_id;
    }
}
