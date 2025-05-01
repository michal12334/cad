use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct TorusDeleted {
    pub id: u64,
}
