#![doc = include_str!("../README.md")]

mod lending_iter;
mod lending_iter_mut;
mod popping_iter;

pub use lending_iter::LendingIter;
pub use lending_iter_mut::LendingIterMut;
pub use popping_iter::PoppingIter;

/// An [extension trait] for `Vec<Vec<T>>`.
///
/// [extension trait]: https://doc.rust-lang.org/book/ch10-02-traits.html#extending-a-trait
pub trait VecVecExt {
    /// The type `T` of the items contained in the `Vec<Vec<T>>`.
    type Item;

    /// Returns a [`PoppingIter`] over the `Vec<Vec<T>>`.
    fn popping_iter(&mut self) -> PoppingIter<'_, Self::Item>;

    /// Returns a [lending iterator] over the mutable references to the elements of `Vec<Vec<T>>`.
    ///
    /// [lending iterator]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
    fn lending_iter_mut(&mut self) -> LendingIterMut<'_, Self::Item>;

    /// Returns a [lending iterator] over the shared references to the elements of `Vec<Vec<T>>`.
    fn lending_iter(&self) -> LendingIter<'_, Self::Item>;
}

impl<T> VecVecExt for Vec<Vec<T>> {
    type Item = T;

    fn popping_iter(&mut self) -> PoppingIter<'_, Self::Item> {
        PoppingIter(self)
    }

    fn lending_iter_mut(&mut self) -> LendingIterMut<'_, Self::Item> {
        LendingIterMut::new(self)
    }

    fn lending_iter(&self) -> LendingIter<'_, Self::Item> {
        LendingIter::new(self)
    }
}
