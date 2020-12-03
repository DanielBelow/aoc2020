pub trait IteratorExt<T> {
    fn count_if<P>(self, predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(T) -> bool;
}

impl<T: Iterator> IteratorExt<T::Item> for T {
    #[inline]
    fn count_if<P>(self, mut predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(T::Item) -> bool,
    {
        self.fold(0, |acc, it| if predicate(it) { acc + 1 } else { acc })
    }
}
