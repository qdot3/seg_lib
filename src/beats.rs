#![deprecated = "UNDER CONSTRUCTION"]

use crate::traits::{Monoid, MonoidAction};

pub struct SegmentTreeBeats<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Query, Set = Update>,
{
    data: Box<[<Query as Monoid>::Set]>,
    lazy: Box<[<Update as Monoid>::Set]>,
}
