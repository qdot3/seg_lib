/*!
Predefined monoid actions.

Use [`AssignSegmentTree`](crate::assign::AssignSegmentTree) for range assign update.
*/

use std::marker::PhantomData;

use num_integer::Integer;
use num_traits::{FromPrimitive, One, Zero};

use crate::{
    Monoid, MonoidAction,
    ops::{Add, Affine, GCD, LCM, Max, Min, Mul},
};

fn convert_size<T>(size: usize) -> T
where
    T: FromPrimitive,
{
    T::from_usize(size).expect("the Set should be large enough to represent segment size.")
}

pub struct AddQueryAddUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for AddQueryAddUpdate<T>
where
    T: Zero + FromPrimitive,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Map = Add<T>;
    type Set = Add<T>;

    const USE_SEGMENT_SIZE: bool = true;

    /// # Panic
    ///
    /// Panics if `T` is too small to represent the segment size.
    fn act(
        mapping: &<Self::Map as crate::Monoid>::Set,
        element: &<Self::Set as crate::Monoid>::Set,
        size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        let size: T = convert_size(size.unwrap());
        &(mapping * &size) + element
    }
}

pub struct AddQueryAffineUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for AddQueryAffineUpdate<T>
where
    T: One + Zero + FromPrimitive,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Map = Affine<T>;
    type Set = Add<T>;

    const USE_SEGMENT_SIZE: bool = true;

    /// # Panic
    ///
    /// Panics if `T` is too small to represent the segment size.
    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        let size: T = convert_size(size.unwrap());
        &mapping[0] * element + &size * &mapping[1]
    }
}

pub struct AddQueryMulUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for AddQueryMulUpdate<T>
where
    T: One + Zero + FromPrimitive,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Map = Mul<T>;
    type Set = Add<T>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        mapping * element
    }
}

pub struct GCDQueryMulUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for GCDQueryMulUpdate<T>
where
    T: Integer,
    for<'a> &'a T: std::ops::Mul<Output = T>,
{
    type Map = Mul<T>;
    type Set = GCD<T>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        mapping * element
    }
}

pub struct LCMQueryMulUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for LCMQueryMulUpdate<T>
where
    T: Integer,
    for<'a> &'a T: std::ops::Mul<Output = T>,
{
    type Map = Mul<T>;
    type Set = LCM<T>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        mapping * element
    }
}

pub struct MaxQueryAddUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for MaxQueryAddUpdate<T>
where
    T: Clone + Zero,
    for<'a> &'a T: Ord + std::ops::Add<Output = T>,
{
    type Map = Add<T>;
    type Set = Max<T>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        element.as_ref().map(|element| mapping + element)
    }
}

pub struct MinQueryAddUpdate<T>(PhantomData<T>);

impl<T> MonoidAction for MinQueryAddUpdate<T>
where
    T: Clone + Zero,
    for<'a> &'a T: Ord + std::ops::Add<Output = T>,
{
    type Map = Add<T>;
    type Set = Min<T>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        element.as_ref().map(|element| mapping + element)
    }
}
