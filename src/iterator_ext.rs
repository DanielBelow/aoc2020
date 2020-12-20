pub trait IteratorExt {
    type Elem;

    fn count_if<P>(self, predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(Self::Elem) -> bool;

    fn sum_by<F, K>(self, key: F) -> K
    where
        Self: Sized,
        K: std::iter::Sum,
        F: FnMut(Self::Elem) -> K;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    type Elem = T::Item;

    #[inline]
    fn count_if<P>(self, mut predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(Self::Elem) -> bool,
    {
        self.fold(0, |acc, it| if predicate(it) { acc + 1 } else { acc })
    }

    fn sum_by<F, K>(self, key: F) -> K
    where
        Self: Sized,
        K: std::iter::Sum,
        F: FnMut(Self::Elem) -> K,
    {
        self.map(key).sum()
    }
}
