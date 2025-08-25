use std::marker::PhantomData;

use num_traits::bounds::UpperBounded;

use crate::traits::Monoid;

/// Represents `chmin` operation.
pub struct Min<T>(PhantomData<T>);

impl<T> Monoid for Min<T>
where
    T: Clone + UpperBounded,
    for<'a> &'a T: Ord,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        T::max_value()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev.min(rhs_or_new).clone()
    }
}
