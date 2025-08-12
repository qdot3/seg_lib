use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use num_traits::{One, Zero};

use crate::traits::{Query, Update};

#[derive(Debug)]
pub struct Affine<T>(PhantomData<T>);

impl<T> Query<(T, T)> for Affine<T>
where
    T: One + Zero,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    for<'a> T: Add<&'a T, Output = T>,
{
    fn identity() -> (T, T) {
        (T::one(), T::zero())
    }

    fn combine(lhs: &(T, T), rhs: &(T, T)) -> (T, T) {
        // R @ L = (ax + b) @ (cx + d) = a(cx + d) + b = (ac)x + (ad + b)
        (&rhs.0 * &lhs.0, &rhs.0 * &lhs.1 + &rhs.1)
    }
}

impl<T> Update<(T, T)> for Affine<T>
where
    T: One + Zero,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    for<'a> T: Add<&'a T, Output = T>,
{
    type Set = T;

    fn identity() -> (T, T) {
        <Self as Query<(T, T)>>::identity()
    }

    fn combine(previous: &(T, T), new: &(T, T)) -> (T, T) {
        <Self as Query<(T, T)>>::combine(previous, new)
    }

    fn update(op: &(T, T), arg: &Self::Set) -> Self::Set {
        &op.0 * arg + &op.1
    }
}
