pub trait IteratorExtensions<TItem> {
    fn chain_if<TOther>(self, other: TOther, condition: bool) -> impl Iterator<Item = TItem>
    where
        Self: Sized,
        TOther: Iterator<Item = TItem>;
}

impl<T, TItem> IteratorExtensions<TItem> for T
where
    T: Iterator<Item = TItem>,
{
    fn chain_if<TOther>(self, other: TOther, condition: bool) -> impl Iterator<Item = TItem>
    where
        TOther: Iterator<Item = TItem>,
    {
        if condition {
            let v = other.collect::<Vec<_>>();
            self.chain(v.into_iter())
        } else {
            let v = vec![];
            self.chain(v.into_iter())
        }
    }
}
