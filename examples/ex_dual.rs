use seg_lib::{DualSegmentTree, Monoid, ops::Affine};

/// Demonstrates how to use a [`DualSegmentTree`] for:
/// - point queries (direct or functional)
/// - range affine updates
fn main() {
    let mut dual = DualSegmentTree::<Affine<i32>>::new(100);
    assert_eq!(dual.len(), 100);
    assert!(dual.iter().all(|e| *e == Affine::<i32>::identity()));

    // Apply affine transformations
    dual.range_update(..75, &[2, 3]); // ax + b -> 2(ax + b) + 3
    dual.range_update(25.., &[5, 7]); // ax + b -> 5(ax + b) + 7

    // Query a single element
    assert_eq!(dual.point_query(50), [10, 22]); // composed transformation

    // Query with a custom function
    assert_eq!(
        dual.point_query_with(75, |[tilt, offset]| tilt * 100 + offset),
        507
    );

    // Update a single element
    dual.point_update(50, &[0, 100]); // ax + b -> 0(ax + b) + 100
    assert_eq!(dual.point_query(50), [0, 100]);
}
