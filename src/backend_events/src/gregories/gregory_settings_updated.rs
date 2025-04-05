use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct GregorySettingsUpdated {
    pub gregory_id: u64,
    pub tess_level: u8,
    pub draw_vectors: bool,
}
