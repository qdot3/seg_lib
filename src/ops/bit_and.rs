use std::marker::PhantomData;

use num_traits::One;

use crate::traits::Monoid;

/// Represents `&` operation.
pub struct BitAnd<T>(PhantomData<T>);
impl<T> Monoid for BitAnd<T>
where
    T: One,
    for<'a> &'a T: std::ops::BitAnd<Output = T>,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        T::one()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev & rhs_or_new
    }
}
