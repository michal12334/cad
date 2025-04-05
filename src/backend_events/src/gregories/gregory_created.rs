use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct GregoryCreated {
    pub gregory_id: u64,
    pub name: String,
    pub tess_level: u8,
    pub draw_vectors: bool,
}
