pub trait Consumer<T> {
    fn consume(&self, message: &T);
}
