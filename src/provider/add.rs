use std::marker::PhantomData;

use num_traits::Zero;

use crate::traits::{Monoid};

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

    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs + rhs
    }
}

