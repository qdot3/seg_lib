use std::marker::PhantomData;

use num_traits::Zero;

use crate::traits::{Monoid, MonoidAction};

/// Represents `+` operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

// impl<M, S> MonoidAction for Add<(M, S)>
// where
//     M: Zero,
//     for<'a> &'a M: std::ops::Add<Output = M>,
//     for<'a, 'b> &'a M: std::ops::Add<&'b S, Output = S>,
// {
//     type Map = M;

//     type Set = S;

//     const IS_COMMUTATIVE: bool = true;

//     fn identity() -> Self::Map {
//         M::zero()
//     }

//     fn combine(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map {
//         lhs + rhs
//     }

//     fn apply(mapping: &Self::Map, element: &Self::Set) -> Self::Set {
//         mapping + element
//     }
// }
