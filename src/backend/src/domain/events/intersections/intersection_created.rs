use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct IntersectionCreated {
    pub id: u64,
    pub name: String,
}
