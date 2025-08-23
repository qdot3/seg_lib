use std::marker::PhantomData;

use crate::traits::{Monoid, MonoidAction};

/// A data structure which supports *range update range query* operation.
pub struct LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: MonoidAction<Set = <Query as Monoid>::Set>,
{
    data: Box<[<Query as Monoid>::Set]>,
    lazy: Box<[<Update as MonoidAction>::Map]>,

    // for debug
    query: PhantomData<Query>,
    update: PhantomData<Update>,
}

impl<Query, Update> LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: MonoidAction<Set = <Query as Monoid>::Set>,
{
}
