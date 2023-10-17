use stack_trait::Stack;

/// An iterator over the items of a `Vec<Vec<T>>` that
/// pops the elements of the inner `Vec`s from the back.
///
/// ## Example
///
/// ```rust
/// use vec_vec::VecVecExt;
///
/// fn main() {
///      let mut v = vec![vec![2, 3, 5], vec![], vec![7, 11, 13]];
///      let mut iter = v.popping_iter();
///     
///      assert_eq!(iter.next(), Some(13));
///      drop(iter);
///      assert_eq!(v, vec![vec![2, 3, 5], vec![], vec![7, 11]]);
///     
///      let mut iter = v.popping_iter();
///      assert_eq!(iter.next(), Some(11));
///      assert_eq!(iter.container(), &vec![vec![2, 3, 5], vec![], vec![7]]);
///     
///      assert_eq!(iter.next(), Some(7));
///      assert_eq!(iter.container(), &vec![vec![2, 3, 5], vec![], vec![]]);
///     
///      assert_eq!(iter.next(), Some(5));
///      assert_eq!(iter.container(), &vec![vec![2, 3]]);
///     
///      assert_eq!(iter.next(), Some(3));
///      assert_eq!(iter.container(), &vec![vec![2]]);
///     
///      assert_eq!(iter.next(), Some(2));
///      assert_eq!(iter.container(), &vec![vec![]]);
///     
///      assert_eq!(iter.next(), None);
///      assert_eq!(iter.container(), &Vec::<Vec<_>>::new());
/// }
/// ```
pub struct PoppingIter<'a, T>(pub(crate) &'a mut Vec<Vec<T>>);

impl<'a, T> PoppingIter<'a, T> {
    /// Returns a shared reference from the inner reference to the `Vec<Vec<T>>`.
    pub fn container(&self) -> &'_ Vec<Vec<T>> {
        self.0
    }

    /// Returns a mutable reference from the inner reference to the `Vec<Vec<T>>`.
    pub fn container_mut(&mut self) -> &'_ mut Vec<Vec<T>> {
        self.0
    }
}

impl<'a, T> Iterator for PoppingIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut last_inner = self.0.lifo()?;
            if last_inner.is_empty() {
                last_inner.pop_pointee();
                continue;
            };
            let last_inner: &mut Vec<T> = last_inner.as_mut();
            return last_inner.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut v = vec![vec![2, 3, 5], vec![], vec![7, 11, 13]];
        let mut iter = v.popping_iter();

        assert_eq!(iter.next(), Some(13));
        drop(iter);
        assert_eq!(v, vec![vec![2, 3, 5], vec![], vec![7, 11]]);

        let mut iter = v.popping_iter();
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.container(), &vec![vec![2, 3, 5], vec![], vec![7]]);

        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.container(), &vec![vec![2, 3, 5], vec![], vec![]]);

        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.container(), &vec![vec![2, 3]]);

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.container(), &vec![vec![2]]);

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.container(), &vec![vec![]]);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.container(), &Vec::<Vec<_>>::new());
    }
}
