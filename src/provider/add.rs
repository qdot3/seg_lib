use num_traits::Zero;

use std::ops::Add;

use crate::traits::{Query, Update};

#[derive(Debug)]
pub struct AddProvider;

impl<T> Query<T> for AddProvider
where
    T: Zero,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    fn identity() -> T {
        T::zero()
    }

    fn combine(lhs: &T, rhs: &T) -> T {
        lhs.add(rhs)
    }
}

impl<T> Update<T> for AddProvider
where
    T: Zero,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Arg = T;

    fn identity() -> T {
        T::zero()
    }

    fn combine(previous: &T, new: &T) -> T {
        previous.add(new)
    }

    fn update(op: &T, arg: &Self::Arg) -> Self::Arg {
        op.add(arg)
    }
}
