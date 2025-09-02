use std::marker::PhantomData;

use num_traits::Zero;

use crate::{MonoidAction, traits::Monoid};

/// Represents `+` operation.
// ANCHOR: def_and_impl_monoid
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

impl<T> MonoidAction for (Add<T>, Add<T>)
where
    T: Zero,
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    type Map = Add<T>;

    type Set = Add<T>;

    const USE_SEGMENT_SIZE: bool = true;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        todo!()
    }
}
