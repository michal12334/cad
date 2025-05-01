use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct TorusTransformed {
    pub id: u64,
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64, f64),
    pub scale: (f64, f64, f64),
}
