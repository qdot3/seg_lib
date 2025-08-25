use std::marker::PhantomData;

use num_traits::bounds::LowerBounded;

use crate::traits::Monoid;

/// Represents `chmax` operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Max<T>(PhantomData<T>);

impl<T> Monoid for Max<T>
where
    T: Clone + LowerBounded,
    for<'a> &'a T: Ord,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        T::min_value()
    }

    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs.min(rhs).clone()
    }
}
