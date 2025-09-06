use seg_lib::{LazySegmentTree, acts::AddQueryMulUpdate};

/// Demonstrates how to use a [`LazySegmentTree`] for:
/// - range sum queries
/// - range multiplication updates
fn main() {
    // Initialize a lazy segment tree with values 0..100
    let mut seg = LazySegmentTree::<AddQueryMulUpdate<i32>>::from_iter(0..100);
    assert_eq!(seg.len(), 100);
    assert_eq!(Vec::from_iter(seg.iter().copied()), Vec::from_iter(0..100));

    // Apply a range update: x -> 2x
    seg.range_update(.., &2);
    assert_eq!(
        Vec::from_iter(seg.iter().copied()),
        (0..100).map(|i| i * 2).collect::<Vec<_>>()
    );

    // Query the sum over the first 50 elements
    assert_eq!(seg.range_query(..50), (0..50).map(|i| 2 * i).sum());

    // Update and query a single element
    seg.point_update(50, &100);
    assert_eq!(seg.point_query(50), &10_000);
}
