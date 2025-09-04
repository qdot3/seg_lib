use seg_lib::{SegmentTree, ops::Add};

fn main() {
    // creates a segment tree for range add query on i32 initialized with 0,
    // which is the identity element in this case.
    // [0, 0, 0, 0, 0, 0, 0]
    let segment_tree = SegmentTree::<Add<i32>>::new(7);
}
