use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct TorusUpdated {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}
