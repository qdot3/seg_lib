use std::marker::PhantomData;

use crate::traits::Monoid;

pub struct Max<T>(PhantomData<T>);

impl<T> Monoid for Max<T> {
    type Set = T;

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        todo!()
    }

    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        todo!()
    }
}
