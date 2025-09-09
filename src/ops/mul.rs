use std::marker::PhantomData;

use num_traits::One;

use crate::traits::Monoid;

/// Performs `*` operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mul<T>(PhantomData<T>);

impl<T> Monoid for Mul<T>
where
    T: One,
    for<'a> &'a T: std::ops::Mul<Output = T>,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        T::one()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev * rhs_or_new
    }
}
