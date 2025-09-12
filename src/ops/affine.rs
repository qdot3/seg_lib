use std::marker::PhantomData;

use num_traits::{One, Zero};

use crate::traits::Monoid;

/// Performs affine transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Affine<T>(PhantomData<T>);

impl<T> Monoid for Affine<T>
where
    T: One + Zero,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Set = (T, T);

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        (T::one(), T::zero())
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        (
            &rhs_or_new.0 * &lhs_or_prev.0,
            &(&rhs_or_new.0 * &lhs_or_prev.1) + &rhs_or_new.1,
        )
    }
}
