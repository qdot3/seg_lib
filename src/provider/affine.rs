use std::marker::PhantomData;

use num_traits::{One, Zero};

use crate::traits::Monoid;

/// Represents affine transformation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

// impl<M, S> MonoidAction for Affine<(M, S)>
// where
//     M: One + Zero,
//     for<'a> &'a M: std::ops::Add<Output = M> + std::ops::Mul<Output = M>,
//     for<'a, 'b> &'a M: std::ops::Add<&'b S, Output = S> + std::ops::Mul<&'b S, Output = S>,
// {
//     type Map = [M; 2];

//     type Set = S;

//     const IS_COMMUTATIVE: bool = false;

//     fn identity() -> Self::Map {
//         [M::one(), M::zero()]
//     }

//     fn combine(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map {
//         [&lhs[0] * &rhs[0], &(&lhs[0] * &rhs[1]) + &lhs[1]]
//     }

//     fn apply(mapping: &Self::Map, element: &Self::Set) -> Self::Set {
//         &mapping[1] + &(&mapping[0] * element)
//     }
// }
