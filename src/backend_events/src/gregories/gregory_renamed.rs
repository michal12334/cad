use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct GregoryRenamed {
    pub gregory_id: u64,
    pub name: String,
}
