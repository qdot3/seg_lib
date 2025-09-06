use seg_lib::{DualSegmentTree, Monoid, ops::Affine};

/// Demonstrates how to use a [`DualSegmentTree`] for:
/// - point queries (direct or functional)
/// - range affine updates
fn main() {
    // Initialize a dual segment tree with identity transformations
    let mut seg = DualSegmentTree::<Affine<i32>>::new(100);
    assert_eq!(seg.len(), 100);
    assert!(seg.iter().all(|e| *e == Affine::<i32>::identity()));

    // Compose affine transformations
    seg.range_update(..75, &[2, 3]); // ax + b -> 2(ax + b) + 3
    seg.range_update(25.., &[5, 7]); // ax + b -> 5(ax + b) + 7

    // Query a single element
    assert_eq!(seg.point_query(50), [10, 22]);

    // Query with a custom function
    assert_eq!(seg.point_query_with(75, |[a, b]| a * 100 + b), 507);

    // Update and query a single element
    seg.point_update(50, &[0, 100]); // ax + b -> 0(ax + b) + 100
    assert_eq!(seg.point_query(50), [0, 100]);
}
