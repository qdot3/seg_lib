use std::marker::PhantomData;

use crate::traits::Monoid;

/// Represents `=` operation.
pub struct Assign<T>(PhantomData<T>);

impl<T> Monoid for Assign<T>
where
    T: Clone,
{
    type Set = Option<T>;

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        None
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        rhs_or_new.as_ref().or(lhs_or_prev.as_ref()).cloned()
    }
}
