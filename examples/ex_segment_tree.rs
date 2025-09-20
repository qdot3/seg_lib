use seg_lib::{SegmentTree, ops::Add};

/// Demonstrates how to use a [`SegmentTree`] for:
/// - range sum queries
/// - point updates (direct or functional)
fn main() {
    // Initialize a segment tree with 7 elements, all set to 0
    let mut seg = SegmentTree::<Add<i32>>::from_iter(0..10);
    assert_eq!(seg.len(), 10);
    // Iterates all elements in O(N) time
    assert_eq!(seg.iter().sum::<i32>(), (0 + 9) * 10 / 2);

    // Update a single element
    seg.point_update(0, 100);
    assert_eq!(
        Vec::from_iter(seg.iter().copied()),
        vec![100, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    // Query a single element and the sum over the entire range
    assert_eq!(seg.point_query(0), &100);
    assert_eq!(seg.range_query(..), 100 + (1 + 9) * 9 / 2);

    // Update a single element with a custom function
    seg.point_update_with(5, |x| x * 2 + 7);
    assert_eq!(
        Vec::from_iter(seg.iter().copied()),
        vec![100, 1, 2, 3, 4, 17, 6, 7, 8, 9]
    );

    // Performs binary search in O(log N) time
    assert_eq!(seg.partition_end(0, |sum| *sum <= 120), 5);
    assert_eq!(seg.partition_start(8, |sum| *sum <= 100), 1);
}
