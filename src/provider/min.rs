use crate::traits::{Query, Update};

#[derive(Debug)]
pub struct Min;

macro_rules! impl_query {
    ($( $t:ty )*) => {$(
        impl Query<$t> for Min {
            fn identity() -> $t {
                <$t>::MAX
            }

            fn combine(lhs: &$t, rhs: &$t) -> $t {
                *lhs.min(rhs)
            }
        }
    )*};
}

impl_query!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize );

macro_rules! impl_query_float {
    ($( $t:ty )*) => {$(
        impl Query<$t> for Min {
            fn identity() -> $t {
                <$t>::INFINITY
            }

            fn combine(lhs: &$t, rhs: &$t) -> $t {
                lhs.min(*rhs)
            }
        }
    )*};
}

impl_query_float!( f32 f64 );

macro_rules! impl_update {
    ($( $t:ty )*) => {$(
        impl Update<$t> for Min {
            type Set = $t;

            fn identity() -> $t {
                <$t>::MAX
            }

            fn combine(prev: &$t, next: &$t) -> $t {
                *prev.min(next)
            }

            fn update(mapping: &$t, element: &Self::Set) -> Self::Set {
                *mapping.min(element)
            }
        }
    )*};
}

impl_update!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize );

macro_rules! impl_update_float {
    ($( $t:ty )*) => {$(
        impl Update<$t> for Min {
            type Set = $t;

            fn identity() -> $t {
                <$t>::INFINITY
            }

            fn combine(prev: &$t, next: &$t) -> $t {
                prev.min(*next)
            }

            fn update(mapping: &$t, element: &$t) -> $t {
                mapping.min(*element)
            }
        }
    )*};
}

impl_update_float!( f32 f64 );
