use std::marker::PhantomData;

use crate::traits::{Monoid, MonoidAction};

pub struct DynamicLazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    // for debug
    query: PhantomData<Query>,
    update: PhantomData<Update>,
}
