use crate::{Monoid, QuasiMonoidAction};

/// UNDER CONSTRUCTION
pub struct SegmentTreeBeats<Function>
where
    Function: QuasiMonoidAction,
{
    data: Box<[<<Function as QuasiMonoidAction>::Set as Monoid>::Set]>,
    lazy: Box<[<<Function as QuasiMonoidAction>::Map as Monoid>::Set]>,

    /// calculate if [`MonoidAction::USE_SEGMENT_SIZE`] is `true`.
    segment_size: Option<Box<[usize]>>,
}

impl<Function> SegmentTreeBeats<Function>
where
    Function: QuasiMonoidAction,
{
    pub fn range_update(&mut self) {}
}
