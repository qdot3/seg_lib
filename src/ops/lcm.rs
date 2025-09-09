use std::marker::PhantomData;

use num_integer::Integer;

use crate::traits::Monoid;

/// Performs `lcm` operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LCM<T>(PhantomData<T>);

impl<T> Monoid for LCM<T>
where
    T: Integer,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    /// Returns `1`.
    fn identity() -> Self::Set {
        T::one()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev.lcm(rhs_or_new)
    }
}
