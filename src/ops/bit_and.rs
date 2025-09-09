use std::marker::PhantomData;

use num_traits::Zero;

use crate::traits::Monoid;

/// Performs `&` operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitAnd<T>(PhantomData<T>);

impl<T> Monoid for BitAnd<T>
where
    T: Zero + std::ops::Not<Output = T>,
    for<'a> &'a T: std::ops::BitAnd<Output = T>,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        !T::zero()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev & rhs_or_new
    }
}
