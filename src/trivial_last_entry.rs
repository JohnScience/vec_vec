use std::hint::unreachable_unchecked;

// Invariant: The Vec<Vec<T>> is never empty and the last Vec<T> is not empty.
pub struct TrivialLastEntry<'a, T>(&'a mut Vec<Vec<T>>);

impl<'a, T> TrivialLastEntry<'a, T> {
    pub(crate) fn new(vec: &'a mut Vec<Vec<T>>) -> Option<Self> {
        let outer_idx = vec.len().checked_sub(1)?;
        let is_empty = {
            let last_inner_vec = unsafe { vec.get_unchecked(outer_idx) };
            last_inner_vec.is_empty()
        };

        if is_empty {
            None
        } else {
            Some(Self(vec))
        }
    }

    pub fn push_to_outer(&mut self, vec: Vec<T>) -> Option<()> {
        if vec.is_empty() {
            return None;
        };
        self.0.push(vec);
        Some(())
    }
}

impl<'a, T> std::ops::Deref for TrivialLastEntry<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let outer_idx = match self.0.len().checked_sub(1) {
            Some(outer_idx) => outer_idx,
            None => unsafe { unreachable_unchecked() },
        };
        let last_inner_vec = unsafe { self.0.get_unchecked(outer_idx) };
        let inner_idx = last_inner_vec.len().checked_sub(1).unwrap();
        unsafe { last_inner_vec.get_unchecked(inner_idx) }
    }
}

impl<'a, T> std::ops::DerefMut for TrivialLastEntry<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let outer_idx = self.0.len().checked_sub(1).unwrap();
        let last_inner_vec = unsafe { self.0.get_unchecked_mut(outer_idx) };
        let inner_idx = last_inner_vec.len().checked_sub(1).unwrap();
        unsafe { last_inner_vec.get_unchecked_mut(inner_idx) }
    }
}
