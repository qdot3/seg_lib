/*!
`seg_lib` provides segment tree variants.

# Examples

```rust
*/
#![doc = include_str!("../examples/ex_segment_tree.rs")]
/*!
```

See more [examples](https://github.com/qdot3/seg_lib/tree/master/examples).

# Guide

|                            | range query | range update | note                                |
| -------------------------- | ----------- | ------------ | ----------------------------------- |
| [`SegmentTree`]            | ✅           | ❌            |                                     |
| [`DynamicSegmentTree`]     | ✅           | ❌            | large array                         |
| [`DualSegmentTree`]        | ❌           | ✅            |                                     |
| [`LazySegmentTree`]        | ✅           | ✅            |                                     |
| [`DynamicLazySegmentTree`] | ✅           | ✅            | large array                         |
| [`AssignSegmentTree`]      | ✅           | ✅            | specialized for range assign update |

Dynamic dual segment tree will no be implemented because it is useless.
*/

#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]


mod normal;
pub use normal::SegmentTree;

mod dynamic;
pub use dynamic::DynamicSegmentTree;

mod dual;
pub use dual::DualSegmentTree;

mod lazy;
pub use lazy::LazySegmentTree;

mod dynamic_lazy;
pub use dynamic_lazy::DynamicLazySegmentTree;

mod assign;
pub use assign::AssignSegmentTree;

mod beats;
// pub use beats::SegmentTreeBeats;

pub mod acts;

pub mod ops;

mod traits;
pub use traits::{Monoid, MonoidAction, QuasiMonoidAction};

pub(crate) mod utility;
