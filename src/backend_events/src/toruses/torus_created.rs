use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct TorusCreated {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64, f64),
    pub scale: (f64, f64, f64),
}
