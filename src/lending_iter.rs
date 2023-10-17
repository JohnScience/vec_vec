use lending_iterator::prelude::*;
use nougat::gat;

/// A [lending iterator] over the entries of a `Vec<Vec<T>>`.
/// In this case, entry implies that it can be dereferenced either as a shared or a mutable reference.
///
/// [lending iterator]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
pub struct LendingIter<'a, T> {
    container: &'a Vec<Vec<T>>,
    outer_idx: usize,
    inner_idx: usize,
    is_finished: bool,
}

impl<'a, T> LendingIter<'a, T> {
    pub(crate) fn new(container: &'a Vec<Vec<T>>) -> Self {
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
}

#[gat]
impl<'a, T> LendingIterator for LendingIter<'a, T> {
    type Item<'next>
    where
        Self: 'next,
    = &'next T;

    fn next<'next>(self: &'next mut LendingIter<'a, T>) -> Option<&'next T> {
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
                        return Some(unsafe { c.get_unchecked(old_oi).get_unchecked(old_ii) });
                    };
                    if !unsafe { c.get_unchecked(*oi) }.is_empty() {
                        break;
                    }
                }
                0
            }
        };

        // safety: `oi` is less than the length of `c` and `ii` is less than the length of `c[*oi]`
        Some(unsafe { c.get_unchecked(old_oi).get_unchecked(old_ii) })
    }
}

#[cfg(test)]
mod tests {
    use crate::VecVecExt;
    use lending_iterator::prelude::*;

    #[test]
    fn general_case() {
        let v = vec![vec![2, 3, 5], vec![], vec![7, 11, 13]];
        let mut iter = v.lending_iter();
        let mut flat_iter = [2, 3, 5, 7, 11, 13].iter();

        loop {
            let next = iter.next();
            let flat_next = flat_iter.next();
            assert_eq!(next, flat_next);
            if next.is_none() {
                break;
            };
        }
    }

    #[test]
    fn for_empty_outer() {
        let v = Vec::<Vec<i32>>::new();
        let mut iter = v.lending_iter();
        let mut flat_iter = std::iter::empty::<&i32>();

        loop {
            let next = iter.next();
            let flat_next = flat_iter.next();
            assert_eq!(next, flat_next);
            if next.is_none() {
                break;
            };
        }
    }

    #[test]
    fn for_empty_inner() {
        let v: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        let mut iter = v.lending_iter();
        let mut flat_iter = std::iter::empty::<&i32>();

        loop {
            let next = iter.next();
            let flat_next = flat_iter.next();
            assert_eq!(next, flat_next);
            if next.is_none() {
                break;
            };
        }
    }
}
