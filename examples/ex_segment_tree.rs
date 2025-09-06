use seg_lib::{SegmentTree, ops::Add};

/// Demonstrates how to use a [`SegmentTree`] for:
/// - range addition queries
/// - point updates (direct or functional)
fn main() {
    // Create a segment tree on i32, initialized with 0 (the identity element for addition).
    let mut segment_tree = SegmentTree::<Add<i32>>::new(7);
    assert_eq!(segment_tree.len(), 7);
    assert!(segment_tree.iter().all(|e| *e == 0));

    // Replace the 0th element with `100`
    segment_tree.point_update(0, 100);
    assert_eq!(
        Vec::from_iter(segment_tree.iter().copied()),
        vec![100, 0, 0, 0, 0, 0, 0]
    );

    // Query a single element and the sum of all elements
    assert_eq!(segment_tree.point_query(0), &100);
    assert_eq!(segment_tree.range_query(..), 100);

    // Update the 0th element using a custom function
    segment_tree.point_update_with(0, |element| element * 2 + 7);
    assert_eq!(
        Vec::from_iter(segment_tree.iter().copied()),
        vec![207, 0, 0, 0, 0, 0, 0]
    );
}
