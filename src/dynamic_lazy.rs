use std::{marker::PhantomData, ops::Range};

use crate::traits::{Monoid, MonoidAction};

/// A data structure that supports **range query range update** operations on large array.
pub struct DynamicLazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    data: Vec<<Query as Monoid>::Set>,
    range: Range<isize>,

    // for debug
    query: PhantomData<Query>,
    update: PhantomData<Update>,
}
