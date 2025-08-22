use std::marker::PhantomData;

use num_traits::{One, Zero};

use crate::traits::Monoid;

#[derive(Debug)]
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

    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        [&lhs[0] * &rhs[0], &(&lhs[0] * &rhs[1]) + &lhs[1]]
    }
}
