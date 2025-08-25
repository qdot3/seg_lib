use std::marker::PhantomData;

use num_traits::{One, Zero};

use crate::traits::Monoid;

/// Represents affine transformation.
pub struct Affine<T>(PhantomData<T>);

impl<T> Monoid for Affine<T>
where
    T: One + Zero,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Set = [T; 2];

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        [T::one(), T::zero()]
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        [
            &lhs_or_prev[0] * &rhs_or_new[0],
            &(&lhs_or_prev[0] * &rhs_or_new[1]) + &lhs_or_prev[1],
        ]
    }
}
