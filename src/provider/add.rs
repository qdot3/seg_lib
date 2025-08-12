use crate::traits::{Query, Update};

#[derive(Debug)]
pub struct Add;

macro_rules! impl_query {
    ($( $t:ty )*) => {$(
        impl Query<$t> for Add {
            fn identity() -> $t {
                const ZERO: $t = 0 as $t;
                ZERO
            }

            fn combine(lhs: &$t, rhs: &$t) -> $t {
                lhs + rhs
            }
        }
    )*};
}

impl_query!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 );

macro_rules! impl_update {
    ($( $t:ty )*) => {$(
        impl Update<$t> for Add {
            type Set = $t;

            fn identity() -> $t {
                const ZERO: $t = 0 as $t;
                ZERO
            }

            fn combine(prev: &$t, next: &$t) -> $t {
                prev + next
            }

            fn update(mapping: &$t, element: &Self::Set) -> Self::Set {
                mapping + element
            }
        }
    )*};
}

impl_update!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 );
