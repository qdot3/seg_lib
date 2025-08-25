use std::marker::PhantomData;

use num_integer::Integer;

use crate::traits::Monoid;

/// Represents `gcd` operation.
///
/// # Notes
///
/// Following [`Integer`], defines the identity element of `gcd` as `0`.
pub struct GCD<T>(PhantomData<T>);

impl<T> Monoid for GCD<T>
where
    T: Integer,
{
    type Set = T;

    const IS_COMMUTATIVE: bool = true;

    /// Returns `0`, following [`Integer`].
    fn identity() -> Self::Set {
        T::zero()
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        lhs_or_prev.gcd(rhs_or_new)
    }
}
