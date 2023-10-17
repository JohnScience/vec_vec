use lending_iterator::prelude::*;
use nougat::gat;

/// A [lending iterator] over the mutable references to the elements of a `Vec<Vec<T>>`.
///
/// [lending iterator]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
pub struct LendingIterMut<'a, T> {
    container: &'a mut Vec<Vec<T>>,
    outer_idx: usize,
    inner_idx: usize,
    is_finished: bool,
}

impl<'a, T> LendingIterMut<'a, T> {
    pub(crate) fn new(container: &'a mut Vec<Vec<T>>) -> Self {
        let mut oi = 0;
        let mut is_finished = false;
        loop {
            if oi >= container.len() {
                is_finished = true;
                break;
            };
            if !container[oi].is_empty() {
                break;
            };
            oi += 1;
        }
        Self {
            container,
            outer_idx: oi,
            inner_idx: 0,
            is_finished,
        }
    }

    /// Returns a shared reference to the `Vec<Vec<T>>`.
    pub fn container(&self) -> &'_ Vec<Vec<T>> {
        self.container
    }

    /// Returns a mutable reference to the `Vec<Vec<T>>`.
    pub fn container_mut(&mut self) -> &'_ mut Vec<Vec<T>> {
        self.container
    }
}

#[gat]
impl<'a, T> LendingIterator for LendingIterMut<'a, T> {
    type Item<'next>
    where
        Self: 'next,
    = &'next mut T;

    fn next<'next>(self: &'next mut LendingIterMut<'a, T>) -> Option<&'next mut T> {
        let Self {
            container: c,
            outer_idx: oi,
            inner_idx: ii,
            is_finished,
        } = self;
        if *is_finished {
            return None;
        };
        let old_oi = *oi;
        let old_ii = *ii;
        *ii = match ii.checked_add(1) {
            // safety: at the beginning, *oi is 0,
            // And *oi gets modified only in the `_` case, where it is checked to be less than the length of `c`.
            Some(next_ii) if next_ii < unsafe { c.get_unchecked(*oi) }.len() => next_ii,
            _ => {
                loop {
                    *oi = oi.checked_add(1)?;
                    if *oi >= c.len() {
                        *is_finished = true;
                        // safety: `oi` is less than the length of `c` and `ii` is less than the length of `c[*oi]`
                        return Some(unsafe {
                            c.get_unchecked_mut(old_oi).get_unchecked_mut(old_ii)
                        });
                    };
                    if !unsafe { c.get_unchecked(*oi) }.is_empty() {
                        break;
                    }
                }
                0
            }
        };

        // safety: `oi` is less than the length of `c` and `ii` is less than the length of `c[*oi]`
        Some(unsafe { c.get_unchecked_mut(old_oi).get_unchecked_mut(old_ii) })
    }
}

#[cfg(test)]
mod tests {
    use crate::VecVecExt;
    use lending_iterator::prelude::*;

    #[test]
    fn general_case() {
        let mut v = vec![vec![2, 3, 5], vec![], vec![7, 11, 13]];
        let mut iter = v.lending_iter_mut();

        while let Some(x) = iter.next() {
            *x += 1;
        }

        assert_eq!(v, vec![vec![3, 4, 6], vec![], vec![8, 12, 14]]);
    }

    #[test]
    fn for_empty_outer() {
        let mut v = Vec::<Vec<i32>>::new();
        let mut iter = v.lending_iter_mut();

        while let Some(x) = iter.next() {
            *x += 1;
        }

        assert_eq!(v, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn for_empty_inner() {
        let mut v: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        let mut iter = v.lending_iter_mut();

        while let Some(x) = iter.next() {
            *x += 1;
        }

        assert_eq!(v, vec![vec![], vec![], vec![]]);
    }
}
