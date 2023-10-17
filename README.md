# Utilities for the nested vector

[![Crates.io](https://img.shields.io/crates/v/vec_vec)](https://crates.io/crates/vec_vec)
[![Downloads](https://img.shields.io/crates/d/vec_vec.svg)](https://crates.io/crates/vec_vec)
[![Documentation](https://docs.rs/vec_vec/badge.svg)](https://docs.rs/vec_vec)
[![License](https://img.shields.io/crates/l/vec_vec)](https://crates.io/crates/vec_vec)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/vec_vec/status.svg)](https://deps.rs/repo/github/JohnScience/vec_vec)

If you think of using is crate, think again. Generally speaking, `Vec<Vec<T>>` is an antipattern because it is not contiguous -> not cache friendly -> slow. Nearly always, for the buffer you can and should use a newtype arround [`small_vec::SmallVec`] or `Vec<T>`, if possible. For example, if you have a dynamically-sized matrix, you should use the chosen contiguous buffer and maybe some data for dimensions.

However, if you believe that you have a legitimate use case for a nested vector, this crate provides a `PoppingIterator` that you might want to use.

## Example

```rust
use vec_vec::VecVecExt;

fn main() {
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
```

## Also see

* [`stack-trait`] for the stack trait with entry API, which is useful to avoid the limitations of the pre-[Polonius] [NLL] borrow checker.

[`small_vec::SmallVec`]: https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html
[Polonius]: https://www.youtube.com/watch?v=_agDeiWek8w
[NLL]: https://blog.rust-lang.org/2022/08/05/nll-by-default.html
[`stack-trait`]: https://crates.io/crates/stack-trait
