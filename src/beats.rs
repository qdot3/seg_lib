use crate::traits::{Monoid, QuasiMonoidAction};

/// UNDER CONSTRUCTION
#[allow(dead_code)]
pub struct SegmentTreeBeats<Query, Update>
where
    Query: Monoid,
    Update: Monoid + QuasiMonoidAction<Map = Query, Set = Update>,
{
    data: Box<[<Query as Monoid>::Set]>,
    lazy: Box<[<Update as Monoid>::Set]>,
}
