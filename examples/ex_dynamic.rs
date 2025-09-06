use seg_lib::{DynamicSegmentTree, ops::LCM};

/// Demonstrates how to use a [`DynamicSegmentTree`] for:
/// - range LCM queries
/// - point updates (direct or functional)
fn main() {
    // Initialize a dynamic segment tree over -1000..5000
    let range = -1_000..5_000;
    let mut seg = DynamicSegmentTree::<LCM<i32>>::new(range.clone()).unwrap();
    assert_eq!(seg.len(), range.len());
    assert_eq!(seg.range_query(..), 1);

    // Update single elements
    seg.point_update(0, 2 * 3 * 7);
    seg.point_update(1_000, 3 * 7);
    seg.point_update(2_000, 2 * 5);

    // Query elements and ranges
    assert_eq!(seg.point_query(0), 2 * 3 * 7);
    assert_eq!(seg.range_query(..), 2 * 3 * 5 * 7);
    assert_eq!(seg.range_query(..1_000), 2 * 3 * 7);
}
