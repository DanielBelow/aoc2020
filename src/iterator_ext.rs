use std::ops::Add;

pub trait IteratorExt {
    type Elem;

    fn count_if<P>(self, predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(Self::Elem) -> bool;

    fn none<P>(self, predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(Self::Elem) -> bool;

    fn sum_by<F, K>(self, key: F) -> K
    where
        Self: Sized,
        K: Add<Output = K> + Default,
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

    #[inline]
    fn none<P>(mut self, predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(Self::Elem) -> bool,
    {
        !self.any(predicate)
    }

    #[inline]
    fn sum_by<F, K>(self, mut key: F) -> K
    where
        Self: Sized,
        K: Add<Output = K> + Default,
        F: FnMut(Self::Elem) -> K,
    {
        self.fold(K::default(), |acc, it| acc + key(it))
    }
}
