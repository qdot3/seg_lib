use std::marker::PhantomData;

use num_traits::Zero;

use crate::traits::Monoid;

/// Performs `+` operation.
// ANCHOR: def_and_impl_monoid
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Add<T>(PhantomData<T>);

impl<T> Monoid for Add<T>
where
    T: Zero,
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        T::zero()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev + rhs_or_new
    }
}
// ANCHOR_END: def_and_impl_monoid
