/*!
`seg_lib` provides segment tree variants.

# Example

```rust
use seg_lib::{SegmentTree, ops::Add};

// point update range sum query
let mut seg_tree = SegmentTree::<Add<i32>>::from_iter(0..1_000);

assert_eq!(seg_tree.range_query(..), 999 * 1_000 / 2);

// replace 0-th element with 100
seg_tree.point_update(0, 100);
assert_eq!(seg_tree.point_query(0), &100);

// twice 10-th element
seg_tree.point_update_with(10, |v| v * 2);
assert_eq!(seg_tree.point_query(10), &20);
assert_eq!(seg_tree.range_query(..), 999 * 1_000 / 2 + 110)
```

See more [examples](https://github.com/qdot3/seg_lib/tree/master/examples).

# Guide

|                        | range query | range update | note                                |
| ---------------------- | ----------- | ------------ | ----------------------------------- |
| [`SegmentTree`]        | ✅           | ❌            |                                     |
| [`DynamicSegmentTree`] | ✅           | ❌            | large array                         |
| [`DualSegmentTree`]    | ❌           | ✅            |                                     |
| [`LazySegmentTree`]    | ✅           | ✅            |                                     |
| [`AssignSegmentTree`]  | ✅           | ✅            | specialized for range assign update |
*/

mod normal;
pub use normal::SegmentTree;

mod dynamic;
pub use dynamic::DynamicSegmentTree;

mod dual;
pub use dual::DualSegmentTree;

mod lazy;
pub use lazy::LazySegmentTree;

mod dynamic_lazy;

mod assign;
pub use assign::AssignSegmentTree;

mod beats;
// pub use beats::SegmentTreeBeats;

pub mod ops;

mod traits;
pub use traits::{Monoid, MonoidAction, QuasiMonoidAction};
