use std::marker::PhantomData;

use crate::traits::Monoid;

/// Performs `chmin` operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Min<T>(PhantomData<T>);

impl<T> Monoid for Min<T>
where
    T: Clone,
    for<'a> &'a T: Ord,
{
    type Set = Option<T>;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        None
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        match (lhs_or_prev, rhs_or_new) {
            (None, None) => None,
            (None, Some(rhs_or_new)) => Some(rhs_or_new),
            (Some(lhs_or_prev), None) => Some(lhs_or_prev),
            (Some(lhs_or_prev), Some(rhs_or_new)) => Some(lhs_or_prev.min(rhs_or_new)),
        }
        .cloned()
    }
}
