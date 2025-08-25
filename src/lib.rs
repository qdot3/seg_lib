//! `seg_lib` provides segment tree variants.

mod normal;
pub use normal::SegmentTree;

mod dual;
pub use dual::DualSegmentTree;

mod lazy;
pub use lazy::LazySegmentTree;

mod beats;
pub use beats::SegmentTreeBeats;

pub mod ops;

pub mod traits;
