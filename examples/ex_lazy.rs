use seg_lib::{LazySegmentTree, acts::AddQueryMulUpdate};

/// Demonstrates how to use a [`LazySegmentTree`] for:
/// - range sum queries
/// - range multiplication updates
fn main() {
    // Initialize a lazy segment tree with values 0..100
    let mut seg = LazySegmentTree::<AddQueryMulUpdate<i32>>::from_iter(0..100);
    assert_eq!(seg.len(), 100);
    assert_eq!(Vec::from_iter(seg.iter().copied()), Vec::from_iter(0..100));

    // Apply range multiplication updates
    seg.range_update(.., &2); // x -> 2x
    seg.range_update(50..=75, &3); // x -> 3x

    // Query sums over ranges
    assert_eq!(seg.range_query(..50), (0..50).map(|i| 2 * i).sum());
    assert_eq!(seg.range_query(50..=75), (50..=75).map(|i| 6 * i).sum());

    // Update and query a single element
    seg.point_update(50, &100);
    assert_eq!(seg.point_query(50), &30_000);
}
