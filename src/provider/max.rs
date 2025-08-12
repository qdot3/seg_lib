use crate::traits::{Query, Update};

#[derive(Debug)]
pub struct Max;

macro_rules! impl_query {
    ($( $t:ty )*) => {$(
        impl Query<$t> for Max {
            fn identity() -> $t {
                <$t>::MIN
            }

            fn combine(lhs: &$t, rhs: &$t) -> $t {
                *lhs.max(rhs)
            }
        }
    )*};
}

impl_query!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize );

macro_rules! impl_query_float {
    ($( $t:ty )*) => {$(
        impl Query<$t> for Max {
            fn identity() -> $t {
                <$t>::NEG_INFINITY
            }

            fn combine(lhs: &$t, rhs: &$t) -> $t {
                lhs.max(*rhs)
            }
        }
    )*};
}

impl_query_float!( f32 f64 );

macro_rules! impl_update {
    ($( $t:ty )*) => {$(
        impl Update<$t> for Max {
            type Set = $t;

            fn identity() -> $t {
                <$t>::MIN
            }

            fn combine(prev: &$t, next: &$t) -> $t {
                *prev.max(next)
            }

            fn update(mapping: &$t, element: &Self::Set) -> Self::Set {
                *mapping.max(element)
            }
        }
    )*};
}

impl_update!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize );

macro_rules! impl_update_float {
    ($( $t:ty )*) => {$(
        impl Update<$t> for Max {
            type Set = $t;

            fn identity() -> $t {
                <$t>::NEG_INFINITY
            }

            fn combine(prev: &$t, next: &$t) -> $t {
                prev.max(*next)
            }

            fn update(mapping: &$t, element: &$t) -> $t {
                mapping.max(*element)
            }
        }
    )*};
}

impl_update_float!( f32 f64 );
